// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/pass_builder/draw_stream.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 47c4e37f3d243041ae564407a724239de91a9da03f8dafa9b946f24078ccbdb5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_stream.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 39, 150, 163, 228, 388, 398, 402, 412, 415, 424, 433, 436, 441, 449, 453, 462, 464, 472, 473, 482, 487, 490, 500, 1303, 1308, 1321, 1323, 1370, 1385, 1407
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"PDB_registry_path_only","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use frozen; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::DrawPacket::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=48
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55E2
// 首条位置: {"module":388,"inlinee":21986}
// 编译实例 RVAs: 0x6605c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2770296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395264,"raw_hex":"e888bad5ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375395315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t827653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395264,"raw_hex":"e805a10c00","size":5,"soft_fail":false,"target_va":5376222944,"va":5375395286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1970926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395264,"raw_hex":"e8ee121e00","size":5,"soft_fail":false,"target_va":5377366224,"va":5375395293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=37
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":388,"record_offset":32108,"parent_record_offset":32068,"ancestor_record_offset":32068,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":32428,"parent_record_offset":32400,"ancestor_record_offset":32068,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":32480,"parent_record_offset":32456,"ancestor_record_offset":32068,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":32456,"parent_record_offset":32400,"ancestor_record_offset":32068,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":388,"record_offset":32504,"parent_record_offset":32480,"ancestor_record_offset":32068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":32356,"parent_record_offset":32332,"ancestor_record_offset":32068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":388,"record_offset":32528,"parent_record_offset":32504,"ancestor_record_offset":32068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":388,"record_offset":32136,"parent_record_offset":32108,"ancestor_record_offset":32068,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32192,"parent_record_offset":32164,"ancestor_record_offset":32068,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":32280,"parent_record_offset":32248,"ancestor_record_offset":32068,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32400,"parent_record_offset":32248,"ancestor_record_offset":32068,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32164,"parent_record_offset":32136,"ancestor_record_offset":32068,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":32304,"parent_record_offset":32280,"ancestor_record_offset":32068,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":388,"record_offset":32332,"parent_record_offset":32304,"ancestor_record_offset":32068,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":32248,"parent_record_offset":32220,"ancestor_record_offset":32068,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":32220,"parent_record_offset":32192,"ancestor_record_offset":32068,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::DrawPacket::variant
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x690E
// 首条位置: {"module":490,"inlinee":26894}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::DrawPacket::stencil_ref
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x633F
// 首条位置: {"module":441,"inlinee":25407}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$14::default
// 记录形式: inline_instance, inlinee_declaration; instances=49
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55E4
// 首条位置: {"module":388,"inlinee":21988}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":388,"record_offset":32136,"parent_record_offset":32108,"ancestor_record_offset":32108,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":32428,"parent_record_offset":32400,"ancestor_record_offset":32108,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":32480,"parent_record_offset":32456,"ancestor_record_offset":32108,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":388,"record_offset":32456,"parent_record_offset":32400,"ancestor_record_offset":32108,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":388,"record_offset":32504,"parent_record_offset":32480,"ancestor_record_offset":32108,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":388,"record_offset":32356,"parent_record_offset":32332,"ancestor_record_offset":32108,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":388,"record_offset":32528,"parent_record_offset":32504,"ancestor_record_offset":32108,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32192,"parent_record_offset":32164,"ancestor_record_offset":32108,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":32280,"parent_record_offset":32248,"ancestor_record_offset":32108,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32400,"parent_record_offset":32248,"ancestor_record_offset":32108,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":388,"record_offset":32164,"parent_record_offset":32136,"ancestor_record_offset":32108,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":388,"record_offset":32304,"parent_record_offset":32280,"ancestor_record_offset":32108,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":388,"record_offset":32332,"parent_record_offset":32304,"ancestor_record_offset":32108,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":32248,"parent_record_offset":32220,"ancestor_record_offset":32108,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":388,"record_offset":32220,"parent_record_offset":32192,"ancestor_record_offset":32108,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$17::eq
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E52
// 首条位置: {"module":163,"inlinee":11858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::TextureBindingKey::with_view
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x594E
// 首条位置: {"module":398,"inlinee":22862}
// 编译实例 RVAs: 0x6606a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c1ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t398827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395488,"raw_hex":"e8eb150600","size":5,"soft_fail":false,"target_va":5375794336,"va":5375395504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$2::to_texture_binding_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x594C
// 首条位置: {"module":398,"inlinee":22860}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$3::to_texture_binding_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6768
// 首条位置: {"module":464,"inlinee":26472}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":464,"record_offset":25288,"parent_record_offset":25260,"ancestor_record_offset":25260,"inlinee":23118,"source_line":331,"archive":"_pdb/A/modules/00464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$4::to_texture_binding_key
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A59
// 首条位置: {"module":424,"inlinee":23129}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":424,"record_offset":22808,"parent_record_offset":22780,"ancestor_record_offset":22780,"inlinee":23118,"source_line":331,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$8::to_texture_binding_key
// 记录形式: compiled_procedure; instances=1
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":436,"record_offset":14532,"rva":6145584,"va":5374854704}
// 编译实例 RVAs: 0x5dc630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28124180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374854704,"raw_hex":"e81424ad01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374854759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":436,"record_offset":14696,"parent_record_offset":14532,"ancestor_record_offset":14532,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00436.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$22::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68F6
// 首条位置: {"module":473,"inlinee":26870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":473,"record_offset":2796,"parent_record_offset":2744,"ancestor_record_offset":2744,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":2852,"parent_record_offset":2744,"ancestor_record_offset":2744,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":473,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":2744,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":473,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":2744,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":473,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2744,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":473,"record_offset":3152,"parent_record_offset":3096,"ancestor_record_offset":2744,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":473,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2744,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2744,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2744,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":3284,"parent_record_offset":2852,"ancestor_record_offset":2744,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":473,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2744,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":473,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":2744,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":473,"record_offset":3096,"parent_record_offset":2948,"ancestor_record_offset":2744,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":473,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2744,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":473,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":2744,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":473,"record_offset":3028,"parent_record_offset":3000,"ancestor_record_offset":2744,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":473,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2744,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":473,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":2744,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPassRenderCheck>
// 记录形式: compiled_procedure; instances=1
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":742880,"rva":7268800,"va":5375977920}
// 编译实例 RVAs: 0x6ee9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27000925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375977920,"raw_hex":"e85d009c01","size":5,"soft_fail":false,"target_va":5402978944,"va":5375978014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26997371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375977920,"raw_hex":"e87bf29b01","size":5,"soft_fail":false,"target_va":5402975328,"va":5375977952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::impl$9::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":500,"record_offset":743060,"parent_record_offset":742880,"ancestor_record_offset":742880,"inlinee":26890,"source_line":258,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":500,"record_offset":743220,"parent_record_offset":743060,"ancestor_record_offset":742880,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":743152,"parent_record_offset":743128,"ancestor_record_offset":742880,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":743176,"parent_record_offset":743152,"ancestor_record_offset":742880,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":500,"record_offset":743128,"parent_record_offset":743108,"ancestor_record_offset":742880,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":500,"record_offset":743108,"parent_record_offset":743088,"ancestor_record_offset":742880,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":500,"record_offset":743088,"parent_record_offset":743060,"ancestor_record_offset":742880,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$9::drop
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x690A
// 首条位置: {"module":473,"inlinee":26890}
// 编译实例 RVAs: 0x6ee9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27000925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375977920,"raw_hex":"e85d009c01","size":5,"soft_fail":false,"target_va":5402978944,"va":5375978014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26997371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375977920,"raw_hex":"e87bf29b01","size":5,"soft_fail":false,"target_va":5402975328,"va":5375977952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":473,"record_offset":8256,"parent_record_offset":8080,"ancestor_record_offset":8080,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":473,"record_offset":8112,"parent_record_offset":8080,"ancestor_record_offset":8080,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":473,"record_offset":8188,"parent_record_offset":8160,"ancestor_record_offset":8080,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":473,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":8080,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":473,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":8080,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":473,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":8080,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::RasterPass::get_render_pass
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68F7
// 首条位置: {"module":473,"inlinee":26871}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4212,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":473,"record_offset":4296,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":4732,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":4760,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":6824,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":473,"record_offset":5580,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::render_pass::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/render_pass.rs
//     evidence={"module":473,"record_offset":4944,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":26881,"source_line":17,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":4860,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::OccupiedEntry::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":5552,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":26884,"source_line":2865,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":5608,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":4916,"parent_record_offset":4136,"ancestor_record_offset":4136,"inlinee":26604,"source_line":3050,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":4136,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":4136,"inlinee":11795,"source_line":253,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::extend_from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":4136,"inlinee":26876,"source_line":1068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":4136,"inlinee":11793,"source_line":49,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$22::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":4136,"inlinee":26886,"source_line":1031,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$23::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4348,"parent_record_offset":4320,"ancestor_record_offset":4136,"inlinee":26875,"source_line":1046,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$24::extend_from_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5456,"parent_record_offset":5432,"ancestor_record_offset":4136,"inlinee":26888,"source_line":1082,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$25::from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":4136,"inlinee":26873,"source_line":1131,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$26::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4988,"parent_record_offset":4944,"ancestor_record_offset":4136,"inlinee":26883,"source_line":1141,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":7020,"parent_record_offset":6992,"ancestor_record_offset":4136,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::raw_ptr_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4660,"parent_record_offset":4372,"ancestor_record_offset":4136,"inlinee":26878,"source_line":1115,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":473,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":4136,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":473,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":4136,"inlinee":11796,"source_line":72,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":473,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":4136,"inlinee":10364,"source_line":76,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":473,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":4136,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":473,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":4136,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":473,"record_offset":6008,"parent_record_offset":5952,"ancestor_record_offset":4136,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":473,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":4136,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":473,"record_offset":5156,"parent_record_offset":5104,"ancestor_record_offset":4136,"inlinee":4183,"source_line":40,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":473,"record_offset":4412,"parent_record_offset":4372,"ancestor_record_offset":4136,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":473,"record_offset":5016,"parent_record_offset":4988,"ancestor_record_offset":4136,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":473,"record_offset":6424,"parent_record_offset":6392,"ancestor_record_offset":4136,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":473,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":4136,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":473,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":4136,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":5264,"parent_record_offset":5156,"ancestor_record_offset":4136,"inlinee":8101,"source_line":1889,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":4824,"parent_record_offset":4788,"ancestor_record_offset":4136,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":4508,"parent_record_offset":4412,"ancestor_record_offset":4136,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":5512,"parent_record_offset":4944,"ancestor_record_offset":4136,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":5408,"parent_record_offset":5104,"ancestor_record_offset":4136,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":4136,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":6540,"parent_record_offset":6516,"ancestor_record_offset":4136,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":4136,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":4604,"parent_record_offset":4372,"ancestor_record_offset":4136,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":473,"record_offset":4476,"parent_record_offset":4440,"ancestor_record_offset":4136,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":4136,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":473,"record_offset":4440,"parent_record_offset":4412,"ancestor_record_offset":4136,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":473,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":4136,"inlinee":26863,"source_line":270,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":473,"record_offset":4788,"parent_record_offset":4760,"ancestor_record_offset":4136,"inlinee":26879,"source_line":277,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":4136,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":4136,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6608,"parent_record_offset":5660,"ancestor_record_offset":4136,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6048,"parent_record_offset":5732,"ancestor_record_offset":4136,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5660,"parent_record_offset":5632,"ancestor_record_offset":4136,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6468,"parent_record_offset":5660,"ancestor_record_offset":4136,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5732,"parent_record_offset":5700,"ancestor_record_offset":4136,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5912,"parent_record_offset":5732,"ancestor_record_offset":4136,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6076,"parent_record_offset":5732,"ancestor_record_offset":4136,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6108,"parent_record_offset":6076,"ancestor_record_offset":4136,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5700,"parent_record_offset":5660,"ancestor_record_offset":4136,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5788,"parent_record_offset":5732,"ancestor_record_offset":4136,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6368,"parent_record_offset":5700,"ancestor_record_offset":4136,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":473,"record_offset":6276,"parent_record_offset":6076,"ancestor_record_offset":4136,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":473,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":4136,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":4136,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":4136,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6580,"parent_record_offset":5660,"ancestor_record_offset":4136,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":473,"record_offset":5816,"parent_record_offset":5732,"ancestor_record_offset":4136,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":473,"record_offset":6164,"parent_record_offset":6076,"ancestor_record_offset":4136,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":473,"record_offset":5952,"parent_record_offset":5912,"ancestor_record_offset":4136,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":473,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":4136,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":4136,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68EF
// 首条位置: {"module":473,"inlinee":26863}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68FF
// 首条位置: {"module":473,"inlinee":26879}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":4824,"parent_record_offset":4788,"ancestor_record_offset":4788,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::RasterPass::new
// 记录形式: inline_instance, inlinee_declaration; instances=48
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C9F
// 首条位置: {"module":39,"inlinee":7327}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":16032,"parent_record_offset":16000,"ancestor_record_offset":16000,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":402,"record_offset":48992,"parent_record_offset":48964,"ancestor_record_offset":48884,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=18
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":39,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":16000,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color_depth
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":388,"record_offset":31808,"parent_record_offset":31784,"ancestor_record_offset":31728,"inlinee":21982,"source_line":132,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_depth
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":449,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":12200,"inlinee":25792,"source_line":157,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":388,"record_offset":31784,"parent_record_offset":31760,"ancestor_record_offset":31728,"inlinee":21981,"source_line":171,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=18
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":39,"record_offset":16056,"parent_record_offset":16032,"ancestor_record_offset":16000,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":449,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":12200,"inlinee":25791,"source_line":185,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":402,"record_offset":49016,"parent_record_offset":48992,"ancestor_record_offset":48884,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00402.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::RasterPass::new_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":473,"record_offset":2064,"rva":6686416,"va":5375395536}
// 编译实例 RVAs: 0x6606d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da370
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3697887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e82193c7ff","size":5,"soft_fail":false,"target_va":5371700080,"va":5375397962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3125923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e85d4dd0ff","size":5,"soft_fail":false,"target_va":5372271824,"va":5375397742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2771763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8cdb4d5ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375396782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064bc90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-84866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e87eb4feff","size":5,"soft_fail":false,"target_va":5375310992,"va":5375395853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d6b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t482209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8a15b0700","size":5,"soft_fail":false,"target_va":5375880064,"va":5375397850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t505105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e811b50700","size":5,"soft_fail":false,"target_va":5375902656,"va":5375397546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t841842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e872d80c00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375398073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t847337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8e9ed0c00","size":5,"soft_fail":false,"target_va":5376242976,"va":5375395634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407378d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t879989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8756d0d00","size":5,"soft_fail":false,"target_va":5376276688,"va":5375396694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e920f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8590847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8ff158300","size":5,"soft_fail":false,"target_va":5383987440,"va":5375396588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14104b0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10397954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e802a99e00","size":5,"soft_fail":false,"target_va":5385793728,"va":5375395769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28196228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8843dae01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375397127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::pass_resources::PassResources::new_from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-953929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8b771f1ff","size":5,"soft_fail":false,"target_va":5374442560,"va":5375396484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::base_layer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t946162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8f26f0e00","size":5,"soft_fail":false,"target_va":5376342224,"va":5375396057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::layer_count
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t946255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e84f700e00","size":5,"soft_fail":false,"target_va":5376342288,"va":5375396028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureViewDesc::level_count
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t946165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8f56f0e00","size":5,"soft_fail":false,"target_va":5376342160,"va":5375395990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1969459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375395536,"raw_hex":"e8330d1e00","size":5,"soft_fail":false,"target_va":5377366224,"va":5375396760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":473,"record_offset":2456,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":473,"record_offset":2400,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":3776,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":473,"record_offset":2316,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::impl$22::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":473,"record_offset":2744,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":26870,"source_line":207,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":473,"record_offset":2684,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":12651,"source_line":11,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$64::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":473,"record_offset":2624,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":10159,"source_line":370,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$40::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":473,"record_offset":2512,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":26868,"source_line":251,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::raster
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":473,"record_offset":2540,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":26866,"source_line":185,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::raster_read
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":473,"record_offset":2484,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":26605,"source_line":224,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":473,"record_offset":2260,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":9083,"source_line":27,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":473,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":2064,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":473,"record_offset":2796,"parent_record_offset":2744,"ancestor_record_offset":2064,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":473,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":2064,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":473,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2064,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":473,"record_offset":3152,"parent_record_offset":3096,"ancestor_record_offset":2064,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":473,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":2064,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":473,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":2064,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":473,"record_offset":3828,"parent_record_offset":3800,"ancestor_record_offset":2064,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":473,"record_offset":2652,"parent_record_offset":2624,"ancestor_record_offset":2064,"inlinee":10161,"source_line":48,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":473,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2064,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2064,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":2064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":2064,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":2064,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2064,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":473,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":2064,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":473,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":2064,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":473,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":2064,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":473,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":2064,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":3284,"parent_record_offset":2852,"ancestor_record_offset":2064,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":2852,"parent_record_offset":2744,"ancestor_record_offset":2064,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":473,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2064,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":473,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":2064,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":473,"record_offset":3096,"parent_record_offset":2948,"ancestor_record_offset":2064,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":473,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2064,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":473,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":2064,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":473,"record_offset":3028,"parent_record_offset":3000,"ancestor_record_offset":2064,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":473,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2064,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":473,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":2064,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 341
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68F1
// 首条位置: {"module":473,"inlinee":26865}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::handle_mut
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":473,"record_offset":772,"parent_record_offset":728,"ancestor_record_offset":728,"inlinee":26380,"source_line":81,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$40::default
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":473,"record_offset":864,"parent_record_offset":728,"ancestor_record_offset":728,"inlinee":26868,"source_line":251,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::raster
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":473,"record_offset":808,"parent_record_offset":728,"ancestor_record_offset":728,"inlinee":26866,"source_line":185,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":473,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":728,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::RasterPass::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 412
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":473,"record_offset":3980,"rva":6689120,"va":5375398240}
// 编译实例 RVAs: 0x661160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da370
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3700616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e87888c7ff","size":5,"soft_fail":false,"target_va":5371700080,"va":5375400691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3129060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e81c41d0ff","size":5,"soft_fail":false,"target_va":5372271824,"va":5375400879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2650630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8fa8dd7ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375400337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064c8b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-86193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e84faffeff","size":5,"soft_fail":false,"target_va":5375314096,"va":5375400284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t487836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e89c710700","size":5,"soft_fail":false,"target_va":5375888544,"va":5375400703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t501852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e85ca80700","size":5,"soft_fail":false,"target_va":5375902656,"va":5375400799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eaf10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t562104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8b8930800","size":5,"soft_fail":false,"target_va":5375962896,"va":5375400787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ee9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t576979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8d3cd0800","size":5,"soft_fail":false,"target_va":5375977920,"va":5375400936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t839024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e870cd0c00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375400891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407378d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t877234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8b2620d00","size":5,"soft_fail":false,"target_va":5376276688,"va":5375399449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140742a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t922571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8cb130e00","size":5,"soft_fail":false,"target_va":5376322160,"va":5375399584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f2f480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9232369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8f1df8c00","size":5,"soft_fail":false,"target_va":5384631424,"va":5375399050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: draw_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28194269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8dd35ae01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375399086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arrayvec::arrayvec::extend_panic
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t27798028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e80c2aa801","size":5,"soft_fail":false,"target_va":5403196608,"va":5375398575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27578404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e824d0a401","size":5,"soft_fail":false,"target_va":5402978944,"va":5375400535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::RenderPassAttachmentDesc::clear_input
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t824176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e870930c00","size":5,"soft_fail":false,"target_va":5376223024,"va":5375398843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::RenderPassAttachmentDesc::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t824534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8d6940c00","size":5,"soft_fail":false,"target_va":5376222992,"va":5375398453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t824571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e8fb940c00","size":5,"soft_fail":false,"target_va":5376223056,"va":5375398480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27574862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375398240,"raw_hex":"e84ec2a401","size":5,"soft_fail":false,"target_va":5402975328,"va":5375400461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":7084,"parent_record_offset":3980,"ancestor_record_offset":3980,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::get_render_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":473,"record_offset":4136,"parent_record_offset":3980,"ancestor_record_offset":3980,"inlinee":26871,"source_line":266,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":473,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":3980,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":473,"record_offset":7452,"parent_record_offset":7236,"ancestor_record_offset":3980,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":3980,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":3980,"inlinee":11795,"source_line":253,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::extend_from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":3980,"inlinee":26876,"source_line":1068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":3980,"inlinee":11793,"source_line":49,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$22::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":3980,"inlinee":26886,"source_line":1031,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$23::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4348,"parent_record_offset":4320,"ancestor_record_offset":3980,"inlinee":26875,"source_line":1046,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$24::extend_from_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":5456,"parent_record_offset":5432,"ancestor_record_offset":3980,"inlinee":26888,"source_line":1082,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$25::from_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":3980,"inlinee":26873,"source_line":1131,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$26::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4988,"parent_record_offset":4944,"ancestor_record_offset":3980,"inlinee":26883,"source_line":1141,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":7020,"parent_record_offset":6992,"ancestor_record_offset":3980,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4212,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::raw_ptr_add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":473,"record_offset":4660,"parent_record_offset":4372,"ancestor_record_offset":3980,"inlinee":26878,"source_line":1115,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":473,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3980,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":473,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":3980,"inlinee":11796,"source_line":72,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":473,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":3980,"inlinee":10364,"source_line":76,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":473,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":3980,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":473,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":3980,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":473,"record_offset":6008,"parent_record_offset":5952,"ancestor_record_offset":3980,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":473,"record_offset":8256,"parent_record_offset":8080,"ancestor_record_offset":3980,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":473,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":3980,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":473,"record_offset":5156,"parent_record_offset":5104,"ancestor_record_offset":3980,"inlinee":4183,"source_line":40,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":473,"record_offset":4412,"parent_record_offset":4372,"ancestor_record_offset":3980,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":473,"record_offset":4296,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":473,"record_offset":8344,"parent_record_offset":8316,"ancestor_record_offset":3980,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":473,"record_offset":7328,"parent_record_offset":7300,"ancestor_record_offset":3980,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":473,"record_offset":7868,"parent_record_offset":7808,"ancestor_record_offset":3980,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":473,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":3980,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":473,"record_offset":6424,"parent_record_offset":6392,"ancestor_record_offset":3980,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":473,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":3980,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":473,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":3980,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":4732,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":5264,"parent_record_offset":5156,"ancestor_record_offset":3980,"inlinee":8101,"source_line":1889,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":4824,"parent_record_offset":4788,"ancestor_record_offset":3980,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":4508,"parent_record_offset":4412,"ancestor_record_offset":3980,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":473,"record_offset":5512,"parent_record_offset":4944,"ancestor_record_offset":3980,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":5408,"parent_record_offset":5104,"ancestor_record_offset":3980,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":3980,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":6540,"parent_record_offset":6516,"ancestor_record_offset":3980,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":3980,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":473,"record_offset":4604,"parent_record_offset":4372,"ancestor_record_offset":3980,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":473,"record_offset":4476,"parent_record_offset":4440,"ancestor_record_offset":3980,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":3980,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":3980,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":473,"record_offset":4440,"parent_record_offset":4412,"ancestor_record_offset":3980,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":473,"record_offset":8188,"parent_record_offset":8160,"ancestor_record_offset":3980,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":473,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":3980,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":473,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":3980,"inlinee":26863,"source_line":270,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":473,"record_offset":4788,"parent_record_offset":4760,"ancestor_record_offset":3980,"inlinee":26879,"source_line":277,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$9::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":473,"record_offset":8080,"parent_record_offset":8052,"ancestor_record_offset":3980,"inlinee":26890,"source_line":258,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":3980,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":3980,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6608,"parent_record_offset":5660,"ancestor_record_offset":3980,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6048,"parent_record_offset":5732,"ancestor_record_offset":3980,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5660,"parent_record_offset":5632,"ancestor_record_offset":3980,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":7268,"parent_record_offset":7236,"ancestor_record_offset":3980,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6468,"parent_record_offset":5660,"ancestor_record_offset":3980,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":7752,"parent_record_offset":7720,"ancestor_record_offset":3980,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":7180,"parent_record_offset":7156,"ancestor_record_offset":3980,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5732,"parent_record_offset":5700,"ancestor_record_offset":3980,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5912,"parent_record_offset":5732,"ancestor_record_offset":3980,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6076,"parent_record_offset":5732,"ancestor_record_offset":3980,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":7236,"parent_record_offset":7180,"ancestor_record_offset":3980,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6108,"parent_record_offset":6076,"ancestor_record_offset":3980,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":3980,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5700,"parent_record_offset":5660,"ancestor_record_offset":3980,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":5788,"parent_record_offset":5732,"ancestor_record_offset":3980,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6368,"parent_record_offset":5700,"ancestor_record_offset":3980,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":7300,"parent_record_offset":7268,"ancestor_record_offset":3980,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":473,"record_offset":6276,"parent_record_offset":6076,"ancestor_record_offset":3980,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":473,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":3980,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":3980,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":7156,"parent_record_offset":7132,"ancestor_record_offset":3980,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":3980,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":473,"record_offset":6580,"parent_record_offset":5660,"ancestor_record_offset":3980,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":473,"record_offset":5816,"parent_record_offset":5732,"ancestor_record_offset":3980,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":473,"record_offset":6164,"parent_record_offset":6076,"ancestor_record_offset":3980,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":473,"record_offset":5952,"parent_record_offset":5912,"ancestor_record_offset":3980,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":473,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":3980,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":473,"record_offset":5580,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":473,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":3980,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::render_pass::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/render_pass.rs
//     evidence={"module":473,"record_offset":4944,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":26881,"source_line":17,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":4860,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":5552,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":26884,"source_line":2865,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":5608,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":4916,"parent_record_offset":4136,"ancestor_record_offset":3980,"inlinee":26604,"source_line":3050,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":473,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":3980,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":473,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":3980,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":473,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":3980,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":473,"record_offset":8112,"parent_record_offset":8080,"ancestor_record_offset":3980,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 426
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCC7
// 首条位置: {"module":1385,"inlinee":64711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1385,"record_offset":28444,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1385,"record_offset":28756,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1385,"record_offset":31732,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1385,"record_offset":28812,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1385,"record_offset":28148,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1385,"record_offset":28908,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1385,"record_offset":30496,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1385,"record_offset":30380,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1385,"record_offset":30408,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1385,"record_offset":35812,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1385,"record_offset":34988,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1385,"record_offset":28360,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1385,"record_offset":34924,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":38380,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":37764,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":42048,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":35868,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1385,"record_offset":28232,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1385,"record_offset":37708,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1385,"record_offset":28276,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1385,"record_offset":31844,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":1385,"record_offset":28964,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":27522,"source_line":102,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::vao_library::AssetVAOLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/vao_library.rs
//     evidence={"module":1385,"record_offset":30580,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":24607,"source_line":24,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1385,"record_offset":30436,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1385,"record_offset":30296,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::mark_unused
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":28616,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":26891,"source_line":447,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":30464,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$22::track
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":28472,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":61716,"source_line":397,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::polygon_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1385,"record_offset":38260,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":64714,"source_line":627,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::stencil_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1385,"record_offset":38136,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":62169,"source_line":619,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::topology
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1385,"record_offset":38192,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":62155,"source_line":612,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1385,"record_offset":38316,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":64715,"source_line":235,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indexed
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1385,"record_offset":38544,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":64717,"source_line":254,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1385,"record_offset":39064,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":64719,"source_line":325,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indexed_indirect_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1385,"record_offset":39568,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":64725,"source_line":347,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indirect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1385,"record_offset":39296,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":64721,"source_line":275,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_api::RenderPassApi::draw_indirect_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_api.rs
//     evidence={"module":1385,"record_offset":39408,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":64723,"source_line":298,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":31908,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":34428,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":35980,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":36340,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1385,"record_offset":38844,"parent_record_offset":38820,"ancestor_record_offset":27572,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1385,"record_offset":38820,"parent_record_offset":38728,"ancestor_record_offset":27572,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1385,"record_offset":38756,"parent_record_offset":38728,"ancestor_record_offset":27572,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1385,"record_offset":38728,"parent_record_offset":38704,"ancestor_record_offset":27572,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1385,"record_offset":28384,"parent_record_offset":28360,"ancestor_record_offset":27572,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1385,"record_offset":40840,"parent_record_offset":40816,"ancestor_record_offset":27572,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1385,"record_offset":40112,"parent_record_offset":40072,"ancestor_record_offset":27572,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1385,"record_offset":38632,"parent_record_offset":38608,"ancestor_record_offset":27572,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1385,"record_offset":38680,"parent_record_offset":38656,"ancestor_record_offset":27572,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1385,"record_offset":40184,"parent_record_offset":40160,"ancestor_record_offset":27572,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1385,"record_offset":28172,"parent_record_offset":28148,"ancestor_record_offset":27572,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1385,"record_offset":41704,"parent_record_offset":41680,"ancestor_record_offset":27572,"inlinee":11795,"source_line":253,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1385,"record_offset":41680,"parent_record_offset":41656,"ancestor_record_offset":27572,"inlinee":11793,"source_line":49,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1385,"record_offset":41780,"parent_record_offset":41752,"ancestor_record_offset":27572,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1385,"record_offset":28196,"parent_record_offset":28172,"ancestor_record_offset":27572,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1385,"record_offset":41728,"parent_record_offset":41704,"ancestor_record_offset":27572,"inlinee":11796,"source_line":72,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1385,"record_offset":41752,"parent_record_offset":41728,"ancestor_record_offset":27572,"inlinee":10364,"source_line":76,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":38348,"parent_record_offset":38316,"ancestor_record_offset":27572,"inlinee":64716,"source_line":1908,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indexed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":38576,"parent_record_offset":38544,"ancestor_record_offset":27572,"inlinee":64718,"source_line":1707,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indexed_indirect
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":39144,"parent_record_offset":39064,"ancestor_record_offset":27572,"inlinee":64720,"source_line":1728,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indexed_indirect_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":39696,"parent_record_offset":39568,"ancestor_record_offset":27572,"inlinee":64726,"source_line":588,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indirect
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":39376,"parent_record_offset":39296,"ancestor_record_offset":27572,"inlinee":64722,"source_line":1927,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_draw_indirect_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1385,"record_offset":39536,"parent_record_offset":39408,"ancestor_record_offset":27572,"inlinee":64724,"source_line":565,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1385,"record_offset":28932,"parent_record_offset":28908,"ancestor_record_offset":27572,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1385,"record_offset":35364,"parent_record_offset":35336,"ancestor_record_offset":27572,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1385,"record_offset":28524,"parent_record_offset":28500,"ancestor_record_offset":27572,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1385,"record_offset":28500,"parent_record_offset":28472,"ancestor_record_offset":27572,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1385,"record_offset":29908,"parent_record_offset":29884,"ancestor_record_offset":27572,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1385,"record_offset":34696,"parent_record_offset":34668,"ancestor_record_offset":27572,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1385,"record_offset":29460,"parent_record_offset":29436,"ancestor_record_offset":27572,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1385,"record_offset":29544,"parent_record_offset":29520,"ancestor_record_offset":27572,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1385,"record_offset":32012,"parent_record_offset":31988,"ancestor_record_offset":27572,"inlinee":8489,"source_line":696,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1385,"record_offset":32292,"parent_record_offset":32264,"ancestor_record_offset":27572,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1385,"record_offset":32192,"parent_record_offset":32168,"ancestor_record_offset":27572,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1385,"record_offset":32168,"parent_record_offset":32012,"ancestor_record_offset":27572,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32096,"parent_record_offset":32068,"ancestor_record_offset":27572,"inlinee":8426,"source_line":190,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32124,"parent_record_offset":32096,"ancestor_record_offset":27572,"inlinee":8427,"source_line":205,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32068,"parent_record_offset":32044,"ancestor_record_offset":27572,"inlinee":8424,"source_line":183,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32364,"parent_record_offset":32340,"ancestor_record_offset":27572,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32240,"parent_record_offset":32216,"ancestor_record_offset":27572,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32388,"parent_record_offset":32364,"ancestor_record_offset":27572,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32264,"parent_record_offset":32240,"ancestor_record_offset":27572,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32436,"parent_record_offset":32388,"ancestor_record_offset":27572,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1385,"record_offset":32828,"parent_record_offset":32388,"ancestor_record_offset":27572,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1385,"record_offset":29484,"parent_record_offset":29460,"ancestor_record_offset":27572,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1385,"record_offset":35012,"parent_record_offset":34988,"ancestor_record_offset":27572,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1385,"record_offset":34952,"parent_record_offset":34924,"ancestor_record_offset":27572,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1385,"record_offset":28408,"parent_record_offset":28384,"ancestor_record_offset":27572,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1385,"record_offset":40160,"parent_record_offset":40136,"ancestor_record_offset":27572,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1385,"record_offset":40136,"parent_record_offset":40112,"ancestor_record_offset":27572,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1385,"record_offset":41536,"parent_record_offset":41508,"ancestor_record_offset":27572,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1385,"record_offset":28548,"parent_record_offset":28524,"ancestor_record_offset":27572,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1385,"record_offset":38784,"parent_record_offset":38756,"ancestor_record_offset":27572,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1385,"record_offset":32536,"parent_record_offset":32436,"ancestor_record_offset":27572,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1385,"record_offset":32504,"parent_record_offset":32436,"ancestor_record_offset":27572,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1385,"record_offset":29656,"parent_record_offset":29632,"ancestor_record_offset":27572,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":35292,"parent_record_offset":35268,"ancestor_record_offset":27572,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":30264,"parent_record_offset":28964,"ancestor_record_offset":27572,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":39108,"parent_record_offset":39064,"ancestor_record_offset":27572,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":30520,"parent_record_offset":30496,"ancestor_record_offset":27572,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1385,"record_offset":30544,"parent_record_offset":30520,"ancestor_record_offset":27572,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1385,"record_offset":34640,"parent_record_offset":34604,"ancestor_record_offset":27572,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1385,"record_offset":40232,"parent_record_offset":40208,"ancestor_record_offset":27572,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":35580,"parent_record_offset":35076,"ancestor_record_offset":27572,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1385,"record_offset":28324,"parent_record_offset":28300,"ancestor_record_offset":27572,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1385,"record_offset":29820,"parent_record_offset":29796,"ancestor_record_offset":27572,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1385,"record_offset":29796,"parent_record_offset":29772,"ancestor_record_offset":27572,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1385,"record_offset":28876,"parent_record_offset":28812,"ancestor_record_offset":27572,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1385,"record_offset":38484,"parent_record_offset":38416,"ancestor_record_offset":27572,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1385,"record_offset":40208,"parent_record_offset":40184,"ancestor_record_offset":27572,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1385,"record_offset":28844,"parent_record_offset":28812,"ancestor_record_offset":27572,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":38508,"parent_record_offset":38484,"ancestor_record_offset":27572,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":28572,"parent_record_offset":28548,"ancestor_record_offset":27572,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1385,"record_offset":38160,"parent_record_offset":38136,"ancestor_record_offset":27572,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1385,"record_offset":29164,"parent_record_offset":28996,"ancestor_record_offset":27572,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1385,"record_offset":34312,"parent_record_offset":34284,"ancestor_record_offset":27572,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1385,"record_offset":34340,"parent_record_offset":34312,"ancestor_record_offset":27572,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1385,"record_offset":31756,"parent_record_offset":31732,"ancestor_record_offset":27572,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1385,"record_offset":37732,"parent_record_offset":37708,"ancestor_record_offset":27572,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1385,"record_offset":28300,"parent_record_offset":28276,"ancestor_record_offset":27572,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1385,"record_offset":34256,"parent_record_offset":34228,"ancestor_record_offset":27572,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1385,"record_offset":29096,"parent_record_offset":29072,"ancestor_record_offset":27572,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1385,"record_offset":30320,"parent_record_offset":30296,"ancestor_record_offset":27572,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1385,"record_offset":29120,"parent_record_offset":29096,"ancestor_record_offset":27572,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$16::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1385,"record_offset":29956,"parent_record_offset":29932,"ancestor_record_offset":27572,"inlinee":22473,"source_line":47,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$7::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1385,"record_offset":29932,"parent_record_offset":29908,"ancestor_record_offset":27572,"inlinee":22471,"source_line":103,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$5
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":1385,"record_offset":37792,"parent_record_offset":37764,"ancestor_record_offset":27572,"inlinee":64712,"source_line":554,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$7
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":1385,"record_offset":38020,"parent_record_offset":37992,"ancestor_record_offset":27572,"inlinee":64713,"source_line":579,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] frozen::FrozenMap::as_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/frozen/src/lib.rs
//     evidence={"module":1385,"record_offset":28996,"parent_record_offset":28964,"ancestor_record_offset":27572,"inlinee":24144,"source_line":113,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1385,"record_offset":29884,"parent_record_offset":29860,"ancestor_record_offset":27572,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":30628,"parent_record_offset":30604,"ancestor_record_offset":27572,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":30652,"parent_record_offset":30628,"ancestor_record_offset":27572,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":29252,"parent_record_offset":29228,"ancestor_record_offset":27572,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":29228,"parent_record_offset":29204,"ancestor_record_offset":27572,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":37840,"parent_record_offset":37816,"ancestor_record_offset":27572,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":34456,"parent_record_offset":34428,"ancestor_record_offset":27572,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":29860,"parent_record_offset":29724,"ancestor_record_offset":27572,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":36396,"parent_record_offset":36368,"ancestor_record_offset":27572,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":29284,"parent_record_offset":29252,"ancestor_record_offset":27572,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":29772,"parent_record_offset":29748,"ancestor_record_offset":27572,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":29748,"parent_record_offset":29724,"ancestor_record_offset":27572,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":30156,"parent_record_offset":30132,"ancestor_record_offset":27572,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":36768,"parent_record_offset":36460,"ancestor_record_offset":27572,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":30096,"parent_record_offset":29364,"ancestor_record_offset":27572,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":34604,"parent_record_offset":34512,"ancestor_record_offset":27572,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":36460,"parent_record_offset":36424,"ancestor_record_offset":27572,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":30132,"parent_record_offset":29336,"ancestor_record_offset":27572,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":29336,"parent_record_offset":29312,"ancestor_record_offset":27572,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":30712,"parent_record_offset":30652,"ancestor_record_offset":27572,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":29312,"parent_record_offset":29252,"ancestor_record_offset":27572,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":34484,"parent_record_offset":34456,"ancestor_record_offset":27572,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":34576,"parent_record_offset":34512,"ancestor_record_offset":27572,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":34548,"parent_record_offset":34512,"ancestor_record_offset":27572,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":29364,"parent_record_offset":29336,"ancestor_record_offset":27572,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":34512,"parent_record_offset":34484,"ancestor_record_offset":27572,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1385,"record_offset":29604,"parent_record_offset":29576,"ancestor_record_offset":27572,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1385,"record_offset":29632,"parent_record_offset":29604,"ancestor_record_offset":27572,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1385,"record_offset":29692,"parent_record_offset":29576,"ancestor_record_offset":27572,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1385,"record_offset":29576,"parent_record_offset":29364,"ancestor_record_offset":27572,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":29408,"parent_record_offset":29364,"ancestor_record_offset":27572,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":35652,"parent_record_offset":35628,"ancestor_record_offset":27572,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":36424,"parent_record_offset":36396,"ancestor_record_offset":27572,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1385,"record_offset":29724,"parent_record_offset":29364,"ancestor_record_offset":27572,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1385,"record_offset":29436,"parent_record_offset":29364,"ancestor_record_offset":27572,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1385,"record_offset":34668,"parent_record_offset":34604,"ancestor_record_offset":27572,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1385,"record_offset":29520,"parent_record_offset":29364,"ancestor_record_offset":27572,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1385,"record_offset":30004,"parent_record_offset":29364,"ancestor_record_offset":27572,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1385,"record_offset":34724,"parent_record_offset":34668,"ancestor_record_offset":27572,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":41508,"parent_record_offset":41484,"ancestor_record_offset":27572,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":30604,"parent_record_offset":30580,"ancestor_record_offset":27572,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":29204,"parent_record_offset":28964,"ancestor_record_offset":27572,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":37816,"parent_record_offset":37792,"ancestor_record_offset":27572,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":36004,"parent_record_offset":35980,"ancestor_record_offset":27572,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":35524,"parent_record_offset":35076,"ancestor_record_offset":27572,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":36368,"parent_record_offset":36340,"ancestor_record_offset":27572,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":35076,"parent_record_offset":35044,"ancestor_record_offset":27572,"inlinee":18613,"source_line":3127,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$80::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":35552,"parent_record_offset":35076,"ancestor_record_offset":27572,"inlinee":18503,"source_line":3143,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1385,"record_offset":35140,"parent_record_offset":35116,"ancestor_record_offset":27572,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1385,"record_offset":35216,"parent_record_offset":35188,"ancestor_record_offset":27572,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1385,"record_offset":35336,"parent_record_offset":35188,"ancestor_record_offset":27572,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1385,"record_offset":32044,"parent_record_offset":32012,"ancestor_record_offset":27572,"inlinee":8422,"source_line":84,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1385,"record_offset":32340,"parent_record_offset":32012,"ancestor_record_offset":27572,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1385,"record_offset":32216,"parent_record_offset":32192,"ancestor_record_offset":27572,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1385,"record_offset":35116,"parent_record_offset":35076,"ancestor_record_offset":27572,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1385,"record_offset":29048,"parent_record_offset":29024,"ancestor_record_offset":27572,"inlinee":6669,"source_line":36,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1385,"record_offset":29072,"parent_record_offset":29048,"ancestor_record_offset":27572,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":1385,"record_offset":29024,"parent_record_offset":28996,"ancestor_record_offset":27572,"inlinee":24145,"source_line":531,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1385,"record_offset":35240,"parent_record_offset":35216,"ancestor_record_offset":27572,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1385,"record_offset":35268,"parent_record_offset":35240,"ancestor_record_offset":27572,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1385,"record_offset":35188,"parent_record_offset":35164,"ancestor_record_offset":27572,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1385,"record_offset":35164,"parent_record_offset":35140,"ancestor_record_offset":27572,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 437
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF053
// 首条位置: {"module":1308,"inlinee":61523}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 512
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF130
// 首条位置: {"module":1321,"inlinee":61744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1321,"record_offset":8164,"parent_record_offset":8024,"ancestor_record_offset":8024,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":8056,"parent_record_offset":8024,"ancestor_record_offset":8024,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1321,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":8024,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1321,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":8024,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":8088,"parent_record_offset":8056,"ancestor_record_offset":8024,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":8120,"parent_record_offset":8088,"ancestor_record_offset":8024,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 517
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1BB
// 首条位置: {"module":1323,"inlinee":61883}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1323,"record_offset":184236,"parent_record_offset":184212,"ancestor_record_offset":184212,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1323,"record_offset":184284,"parent_record_offset":184260,"ancestor_record_offset":184212,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1323,"record_offset":184260,"parent_record_offset":184236,"ancestor_record_offset":184212,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01323.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 538
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF15B
// 首条位置: {"module":1321,"inlinee":61787}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1321,"record_offset":73000,"parent_record_offset":72972,"ancestor_record_offset":72972,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":73028,"parent_record_offset":73000,"ancestor_record_offset":72972,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":73056,"parent_record_offset":73028,"ancestor_record_offset":72972,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 554
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCC8
// 首条位置: {"module":1385,"inlinee":64712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":37816,"parent_record_offset":37792,"ancestor_record_offset":37792,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":37840,"parent_record_offset":37816,"ancestor_record_offset":37792,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 579
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCC9
// 首条位置: {"module":1385,"inlinee":64713}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1385,"record_offset":38044,"parent_record_offset":38020,"ancestor_record_offset":38020,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1385,"record_offset":38068,"parent_record_offset":38044,"ancestor_record_offset":38020,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/01385.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_stream.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::DrawKind
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::render::pass_builder::draw_stream::DrawKind>", "constructor": "enum2$", "arguments": ["country_core::render::pass_builder::draw_stream::DrawKind"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::render::pass_builder::draw_stream::DrawKind> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::render::pass_builder::draw_stream::DrawKind>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::DrawPacket
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPacket>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPacket"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPacket", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPacket", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::render::pass_builder::draw_stream::DrawPacket,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPacket", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::DrawPassRenderCheck
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPassRenderCheck>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::pass_builder::draw_stream::DrawPassRenderCheck"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::DrawStream
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawStream>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::pass_builder::draw_stream::DrawStream"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::InstancedDraw
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::render::pass_builder::draw_stream::InstancedDraw", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::size_hint<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::size_hint", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>", "constructor": "core::iter::adapters::flatten::and_then_or_clear", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants", "enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants", "core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::RasterPass
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::pass_builder::draw_stream::RasterPass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::draw_stream::TextureBindingKey
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>", "constructor": "tuple$", "arguments": ["country_core::render::pass_builder::draw_stream::TextureBindingKey", "rhapsody::rg::binding::PassResourceSlot"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::render::pass_builder::draw_stream::TextureBindingKey", "rhapsody::rg::binding::PassResourceSlot", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::draw_stream::TextureBindingKey,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_stream.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_stream.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 106; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>", "bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [文件级接口候选，具体函数归属 unknown] alloc::vec::Vec<u16,alloc::alloc::Global> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<std::thread::JoinHandle<tuple$<> >,alloc::alloc::Global> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] i32 (*)(u32,ptr_mut$<windows_sys::Windows::Win32::UI::Input::Pointer::POINTER_PEN_INFO>)
// 候选参数类型: ["u32", "ptr_mut$<windows_sys::Windows::Win32::UI::Input::Pointer::POINTER_PEN_INFO>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<i32 (*)(u32,ptr_mut$<windows_sys::Windows::Win32::UI::Input::Pointer::POINTER_PEN_INFO>)> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPassRenderCheck>
// 信息 ID: 19332cc1d3a8abd01401944d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::ptr::drop_in_place -> ["country_core::render::pass_builder::draw_stream::DrawPassRenderCheck"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass> @ 0x38c9d5 -> 0x6ee9c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::DrawPacket::new
// 信息 ID: f5d20005eacc3337b40c4fd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 36 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d4f3 -> 0x6605c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=32068; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=33232; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=34380; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=38376; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=39540; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=40688; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=42096; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=43272; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::DrawPacket::stencil_ref
// 信息 ID: 1c5b5380fc1dd306765a7d50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=441, inline_record=19044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::DrawPacket::variant
// 信息 ID: f4207b4f82446ad42a60c5b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::DrawBuilder::new; module=490, inline_record=28884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::RasterPass::get_render_pass
// 信息 ID: 233d3e6e1044364326c73ce0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=473, inline_record=4136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::RasterPass::new
// 信息 ID: 223e4b759bcb55163ddf24d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 33 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=16000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=31728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=32876; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=34040; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=38036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=39184; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=40348; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=41760; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::RasterPass::new_impl
// 信息 ID: ddae4187f72e2566c3e2db7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 33 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5ce4f -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b56c -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b834 -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50bae2 -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c33c -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c604 -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c8b2 -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50ce9a -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50d15b -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery @ 0x55585a -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg @ 0x579604 -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay @ 0x579abb -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::menu_background::render_menu_bg @ 0x57a3e9 -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass @ 0x5837ff -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cd053 -> 0x6606d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster @ 0x5ec5e8 -> 0x6606d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::RasterPass::render
// 信息 ID: bcd18ee9f744b945683fa832
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 33 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d800 -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b78f -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50ba52 -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50bcfe -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c55f -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c822 -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50cace -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50d09f -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50d360 -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery @ 0x555f3f -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg @ 0x5797fe -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay @ 0x579d0f -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::menu_background::render_menu_bg @ 0x57a5dc -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass @ 0x5839f0 -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5ce1c2 -> 0x661160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster @ 0x5ece26 -> 0x661160 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::TextureBindingKey::with_view
// 信息 ID: 89e7c779345d496d60f39220
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x358049 -> 0x6606a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>; module=398, inline_record=32216; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >; module=424, inline_record=22868; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=464, inline_record=25348; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$0
// 信息 ID: afc8aca0d028ad8c2b1b841e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$10::get_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=473, inline_record=4556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$10::get_render_pass::closure$1
// 信息 ID: 0a86271005121dd6052c6412
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$10::get_render_pass", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=473, inline_record=4788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure$0
// 信息 ID: d13f9e90a225ae3bcc0d1312
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::new_impl", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ref_mut$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> > >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0 <- arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref_mut$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> > >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0>; module=473, inline_record=728; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1407, inline_record=4408; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1407, inline_record=4608; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$25::from_iter<rhapsody::rg::pass_api::RasterPassColorAttachment,5,core::iter::adapters::map::Map<core::slice::iter::IterMut<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment> >,country_core::render::pass_builder::draw_stream::impl$11::new_impl::closure_env$0> >; module=1407, inline_record=4812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0
// 信息 ID: 081dc36f5a1488e502a459d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0 <- core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0 <- alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=27572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$0
// 信息 ID: fcfd05776a6df86687c4a7e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0> >; module=1308, inline_record=165248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$1
// 信息 ID: 08ed5614b2c3002bf5a63488
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$60::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1 <- hashbrown::map::impl$94::extend<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >; module=1321, inline_record=8024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$2
// 信息 ID: 244caf91efd216aa99978419
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$60::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$60::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2 <- hashbrown::map::impl$94::extend<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$60::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >; module=1323, inline_record=184212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$3
// 信息 ID: 5ec9facb1b1d7fab19682c06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3 <- rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1321, inline_record=72972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$5
// 信息 ID: 1996f961ca7cb782ee3eb2cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=37792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$7
// 信息 ID: dfe62b17d695359c6738dc5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::pass_builder::draw_stream::impl$11::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=38020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$14::default
// 信息 ID: 7cf0069446c46cb7dd025ee8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 37 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=32108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=33272; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=34420; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=38416; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=39580; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=40728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=42136; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=43312; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$17::eq
// 信息 ID: 9bc948da432b69622edff593
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> >; module=228, inline_record=12636; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>; module=398, inline_record=32936; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >; module=424, inline_record=23588; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=464, inline_record=26068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$22::default
// 信息 ID: b2593ab3f1a26e6aa4035c55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=473, inline_record=2744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$2::to_texture_binding_key
// 信息 ID: 2ec05cedd7d0d6a49e8c545d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>; module=398, inline_record=32160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$3::to_texture_binding_key
// 信息 ID: fd2d05fb9e81060d9a71e986
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >; module=464, inline_record=25260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$4::to_texture_binding_key
// 信息 ID: 65fb9b0e6d8bf9df9bf9d050
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >; module=424, inline_record=22780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$8::to_texture_binding_key
// 信息 ID: 5190dcc3e1e7a028dc2d182a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x358030 -> 0x5dc630 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture> > @ 0x64535d -> 0x5dc630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::draw_stream::impl$9::drop
// 信息 ID: 05572412495ad8b4adc0acc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass> @ 0x38c9d5 -> 0x6ee9c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::render; module=473, inline_record=8080; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>; module=500, inline_record=661956; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::DrawPassRenderCheck>; module=500, inline_record=743060; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
