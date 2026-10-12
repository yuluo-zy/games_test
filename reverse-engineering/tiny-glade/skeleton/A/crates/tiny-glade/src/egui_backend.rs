// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/egui_backend.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4665984b36455d2508829499b80e5a889bacf8ceaff8c583e9d2c44200c141be
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: egui_backend.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 39, 127, 256
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
// use ecolor; // namespace candidate
//   package identities: [{"name":"ecolor","source":"PDB_registry_path_only","version":"0.27.2"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"PDB_registry_path_only","version":"0.27.2"}]
// use epaint; // namespace candidate
//   package identities: [{"name":"epaint","source":"PDB_registry_path_only","version":"0.27.2"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBA
// 首条位置: {"module":256,"inlinee":7354}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":256,"record_offset":58444,"parent_record_offset":58400,"ancestor_record_offset":58400,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":256,"record_offset":58716,"parent_record_offset":58688,"ancestor_record_offset":58400,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":256,"record_offset":58768,"parent_record_offset":58744,"ancestor_record_offset":58400,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":256,"record_offset":58744,"parent_record_offset":58688,"ancestor_record_offset":58400,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":256,"record_offset":58792,"parent_record_offset":58768,"ancestor_record_offset":58400,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":256,"record_offset":58644,"parent_record_offset":58620,"ancestor_record_offset":58400,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":256,"record_offset":58816,"parent_record_offset":58792,"ancestor_record_offset":58400,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":256,"record_offset":58492,"parent_record_offset":58468,"ancestor_record_offset":58400,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":256,"record_offset":58568,"parent_record_offset":58540,"ancestor_record_offset":58400,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":256,"record_offset":58688,"parent_record_offset":58540,"ancestor_record_offset":58400,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":256,"record_offset":58468,"parent_record_offset":58444,"ancestor_record_offset":58400,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":256,"record_offset":58592,"parent_record_offset":58568,"ancestor_record_offset":58400,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":256,"record_offset":58620,"parent_record_offset":58592,"ancestor_record_offset":58400,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":256,"record_offset":58540,"parent_record_offset":58516,"ancestor_record_offset":58400,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":256,"record_offset":58516,"parent_record_offset":58492,"ancestor_record_offset":58400,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::take_input
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBB
// 首条位置: {"module":127,"inlinee":7355}
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
// 首条位置: {"module":39,"record_offset":5660,"rva":376544,"va":5369085664}
// 编译实例 RVAs: 0x5bee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c0be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2509846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369085664,"raw_hex":"e8164c2600","size":5,"soft_fail":false,"target_va":5371595744,"va":5369085893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ffc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3227565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369085664,"raw_hex":"e8ad3f3100","size":5,"soft_fail":false,"target_va":5372313536,"va":5369085966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377650
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3257407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369085664,"raw_hex":"e83fb43100","size":5,"soft_fail":false,"target_va":5372343888,"va":5369086476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037aa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3270755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369085664,"raw_hex":"e863e83100","size":5,"soft_fail":false,"target_va":5372357248,"va":5369086488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140381da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3300256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369085664,"raw_hex":"e8a05b3200","size":5,"soft_fail":false,"target_va":5372386720,"va":5369086459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3663695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369085664,"raw_hex":"e84fe73700","size":5,"soft_fail":false,"target_va":5372749712,"va":5369086012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::end_frame
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7421215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369085664,"raw_hex":"e81f3d7100","size":5,"soft_fail":false,"target_va":5376506928,"va":5369085708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] epaint::textures::TexturesDelta::append
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7807499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369085664,"raw_hex":"e80b227700","size":5,"soft_fail":false,"target_va":5376893600,"va":5369086096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":5792,"parent_record_offset":5660,"ancestor_record_offset":5660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":5660,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":6172,"parent_record_offset":6080,"ancestor_record_offset":5660,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":6108,"parent_record_offset":6080,"ancestor_record_offset":5660,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5660,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":5936,"parent_record_offset":5908,"ancestor_record_offset":5660,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":5660,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":5660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":5660,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00039.jsonl.gz"}
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
// 首条位置: {"module":39,"record_offset":6424,"rva":377392,"va":5369086512}
// 编译实例 RVAs: 0x5c230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-371745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8df53faff","size":5,"soft_fail":false,"target_va":5368720672,"va":5369092412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=4
//     target_aliases=11 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e86ed8ffff","size":5,"soft_fail":false,"target_va":5369083440,"va":5369093565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd280
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t527076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8e40a0800","size":5,"soft_fail":false,"target_va":5369614976,"va":5369087895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14026aee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2155428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8a4e32000","size":5,"soft_fail":false,"target_va":5371244256,"va":5369088823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c1780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2506469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8e53e2600","size":5,"soft_fail":false,"target_va":5371598720,"va":5369092246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2559634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8920e2700","size":5,"soft_fail":false,"target_va":5371650384,"va":5369090745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceeb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2565372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8fc242700","size":5,"soft_fail":false,"target_va":5371653808,"va":5369088431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140357bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3122702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e80ea62f00","size":5,"soft_fail":false,"target_va":5372214256,"va":5369091549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035e790
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3152654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e80e1b3000","size":5,"soft_fail":false,"target_va":5372241808,"va":5369089149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036f3f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3214206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e87e0b3100","size":5,"soft_fail":false,"target_va":5372310512,"va":5369096301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140372530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3227038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e89e3d3100","size":5,"soft_fail":false,"target_va":5372323120,"va":5369096077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403861d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3307454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8be773200","size":5,"soft_fail":false,"target_va":5372404176,"va":5369096717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038c7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3333342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8dedc3200","size":5,"soft_fail":false,"target_va":5372430240,"va":5369096893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038db20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3340928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e880fa3200","size":5,"soft_fail":false,"target_va":5372435232,"va":5369094299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038e4f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3345683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8130d3300","size":5,"soft_fail":false,"target_va":5372437744,"va":5369092056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3661976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e898e03700","size":5,"soft_fail":false,"target_va":5372749696,"va":5369087715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3662858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e80ae43700","size":5,"soft_fail":false,"target_va":5372749712,"va":5369086849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=3
//     target_aliases=53 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4059934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e81ef33d00","size":5,"soft_fail":false,"target_va":5373149840,"va":5369089901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7153228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e84c266d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5369086687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7192966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e886c16d00","size":5,"soft_fail":false,"target_va":5376283792,"va":5369090821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140744400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7240418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8e27a6e00","size":5,"soft_fail":false,"target_va":5376328704,"va":5369088281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209ce80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33812862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e87ef10302","size":5,"soft_fail":false,"target_va":5402906240,"va":5369093373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a24e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33839835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8db5a0402","size":5,"soft_fail":false,"target_va":5402928352,"va":5369088512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34502436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e824770e02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369090919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33885245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e83d0c0502","size":5,"soft_fail":false,"target_va":5402978144,"va":5369092894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2215526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e866ce2100","size":5,"soft_fail":false,"target_va":5371307280,"va":5369091749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33885998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e82e0f0502","size":5,"soft_fail":false,"target_va":5402978688,"va":5369092685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33886401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8c1100502","size":5,"soft_fail":false,"target_va":5402978912,"va":5369092506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33886352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e890100502","size":5,"soft_fail":false,"target_va":5402978880,"va":5369092523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33886595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e883110502","size":5,"soft_fail":false,"target_va":5402979152,"va":5369092552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e88cedffff","size":5,"soft_fail":false,"target_va":5369083648,"va":5369088367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<epaint::Primitive> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8c5ccffff","size":5,"soft_fail":false,"target_va":5369083312,"va":5369096422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t33887455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8df140502","size":5,"soft_fail":false,"target_va":5402980128,"va":5369092668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33887744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e800160502","size":5,"soft_fail":false,"target_va":5402980464,"va":5369092715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e871f3ffff","size":5,"soft_fail":false,"target_va":5369083840,"va":5369087050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33887464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8e8140502","size":5,"soft_fail":false,"target_va":5402980656,"va":5369093187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6602975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8dfc06400","size":5,"soft_fail":false,"target_va":5375692416,"va":5369089436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6611155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8d3e06400","size":5,"soft_fail":false,"target_va":5375702768,"va":5369091608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao_indexed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6613206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8d6e86400","size":5,"soft_fail":false,"target_va":5375704896,"va":5369091685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6609336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8b8d96400","size":5,"soft_fail":false,"target_va":5375700688,"va":5369091347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6303944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8c8306000","size":5,"soft_fail":false,"target_va":5375395264,"va":5369091315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6305916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e87c386000","size":5,"soft_fail":false,"target_va":5375395536,"va":5369089615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6306139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e85b396000","size":5,"soft_fail":false,"target_va":5375398240,"va":5369092096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6128735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e85f845d00","size":5,"soft_fail":false,"target_va":5375218240,"va":5369089500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::tessellate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7424345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e859497100","size":5,"soft_fail":false,"target_va":5376514464,"va":5369090114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::ContextImpl::viewport
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7333593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8d9e66f00","size":5,"soft_fail":false,"target_va":5376423568,"va":5369089970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t33873948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e81ce00402","size":5,"soft_fail":false,"target_va":5402963904,"va":5369089951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t33875326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e87ee50402","size":5,"soft_fail":false,"target_va":5402965344,"va":5369090013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7269375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8ffeb6e00","size":5,"soft_fail":false,"target_va":5376356912,"va":5369087532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7273181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8ddfa6e00","size":5,"soft_fail":false,"target_va":5376360560,"va":5369087374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7269103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8efea6e00","size":5,"soft_fail":false,"target_va":5376356144,"va":5369087036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7150119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e8271a6d00","size":5,"soft_fail":false,"target_va":5376240912,"va":5369090788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7252287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e83fa96e00","size":5,"soft_fail":false,"target_va":5376341760,"va":5369089468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7251828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e874a76e00","size":5,"soft_fail":false,"target_va":5376340512,"va":5369088679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7251982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e80ea86e00","size":5,"soft_fail":false,"target_va":5376340720,"va":5369088733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3537773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369086512,"raw_hex":"e86dfb3500","size":5,"soft_fail":false,"target_va":5372625168,"va":5369087390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":6636,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":18752,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":22488,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":18568,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":39,"record_offset":18656,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":39,"record_offset":14432,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":39,"record_offset":18452,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":15936,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":19792,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":39,"record_offset":20508,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":20580,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::push_constants
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":39,"record_offset":21600,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7332,"source_line":120,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":39,"record_offset":22272,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":39,"record_offset":16000,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] egui::context::Context::pixels_per_point
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":39,"record_offset":17640,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":6598,"source_line":1595,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":39,"record_offset":20552,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":39,"record_offset":22400,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":39,"record_offset":22316,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":39,"record_offset":15972,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":18780,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::egui_backend::EguiBackend::release_pending_textures
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":39,"record_offset":23652,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7342,"source_line":218,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::egui_backend::EguiBackend::update_textures
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":39,"record_offset":6904,"parent_record_offset":6424,"ancestor_record_offset":6424,"inlinee":7283,"source_line":164,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":11116,"parent_record_offset":11092,"ancestor_record_offset":6424,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":11140,"parent_record_offset":11116,"ancestor_record_offset":6424,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":7576,"parent_record_offset":7552,"ancestor_record_offset":6424,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":11092,"parent_record_offset":11056,"ancestor_record_offset":6424,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":7552,"parent_record_offset":7464,"ancestor_record_offset":6424,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":7492,"parent_record_offset":7464,"ancestor_record_offset":6424,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":6424,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":11056,"parent_record_offset":11024,"ancestor_record_offset":6424,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":11024,"parent_record_offset":10992,"ancestor_record_offset":6424,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":11212,"parent_record_offset":11024,"ancestor_record_offset":6424,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":7464,"parent_record_offset":7440,"ancestor_record_offset":6424,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":21744,"parent_record_offset":21712,"ancestor_record_offset":6424,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":21712,"parent_record_offset":21680,"ancestor_record_offset":6424,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":21648,"parent_record_offset":21600,"ancestor_record_offset":6424,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":21680,"parent_record_offset":21648,"ancestor_record_offset":6424,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":39,"record_offset":7684,"parent_record_offset":7648,"ancestor_record_offset":6424,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":39,"record_offset":17740,"parent_record_offset":17704,"ancestor_record_offset":6424,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":39,"record_offset":7648,"parent_record_offset":7624,"ancestor_record_offset":6424,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":22532,"parent_record_offset":22488,"ancestor_record_offset":6424,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":6672,"parent_record_offset":6636,"ancestor_record_offset":6424,"inlinee":4126,"source_line":1244,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":7136,"parent_record_offset":7096,"ancestor_record_offset":6424,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10796,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":5202,"source_line":1227,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":7096,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":11388,"parent_record_offset":11364,"ancestor_record_offset":6424,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":7192,"parent_record_offset":7096,"ancestor_record_offset":6424,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":7220,"parent_record_offset":7096,"ancestor_record_offset":6424,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10960,"parent_record_offset":10920,"ancestor_record_offset":6424,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10992,"parent_record_offset":10960,"ancestor_record_offset":6424,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":39,"record_offset":7820,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":39,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":6424,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10856,"parent_record_offset":10824,"ancestor_record_offset":6424,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":6424,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":39,"record_offset":11680,"parent_record_offset":11624,"ancestor_record_offset":6424,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":39,"record_offset":11364,"parent_record_offset":10920,"ancestor_record_offset":6424,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":39,"record_offset":10888,"parent_record_offset":10856,"ancestor_record_offset":6424,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":39,"record_offset":10920,"parent_record_offset":10888,"ancestor_record_offset":6424,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":39,"record_offset":14460,"parent_record_offset":14432,"ancestor_record_offset":6424,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":39,"record_offset":15892,"parent_record_offset":15860,"ancestor_record_offset":6424,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":39,"record_offset":15836,"parent_record_offset":15448,"ancestor_record_offset":6424,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":39,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":6424,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":39,"record_offset":15476,"parent_record_offset":15448,"ancestor_record_offset":6424,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":39,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":6424,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":39,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":6424,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":39,"record_offset":15616,"parent_record_offset":15552,"ancestor_record_offset":6424,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":39,"record_offset":15704,"parent_record_offset":15680,"ancestor_record_offset":6424,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":39,"record_offset":15448,"parent_record_offset":14432,"ancestor_record_offset":6424,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":39,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":6424,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10028,"parent_record_offset":10000,"ancestor_record_offset":6424,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":6424,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10056,"parent_record_offset":10028,"ancestor_record_offset":6424,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":9968,"parent_record_offset":9936,"ancestor_record_offset":6424,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10000,"parent_record_offset":9968,"ancestor_record_offset":6424,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10172,"parent_record_offset":10000,"ancestor_record_offset":6424,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":6424,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":39,"record_offset":15228,"parent_record_offset":15204,"ancestor_record_offset":6424,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":39,"record_offset":15156,"parent_record_offset":15132,"ancestor_record_offset":6424,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":16032,"parent_record_offset":16000,"ancestor_record_offset":6424,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":13708,"parent_record_offset":13684,"ancestor_record_offset":6424,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":6424,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":12092,"parent_record_offset":12068,"ancestor_record_offset":6424,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":39,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":6424,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":39,"record_offset":11436,"parent_record_offset":11412,"ancestor_record_offset":6424,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":39,"record_offset":11252,"parent_record_offset":10920,"ancestor_record_offset":6424,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":39,"record_offset":11488,"parent_record_offset":11460,"ancestor_record_offset":6424,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":10824,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":11412,"parent_record_offset":11388,"ancestor_record_offset":6424,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":11600,"parent_record_offset":11488,"ancestor_record_offset":6424,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":39,"record_offset":18596,"parent_record_offset":18568,"ancestor_record_offset":6424,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":39,"record_offset":10104,"parent_record_offset":10080,"ancestor_record_offset":6424,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":6424,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":6424,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":39,"record_offset":8228,"parent_record_offset":8140,"ancestor_record_offset":6424,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":39,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":6424,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":9860,"parent_record_offset":9832,"ancestor_record_offset":6424,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":15416,"parent_record_offset":14460,"ancestor_record_offset":6424,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":24320,"parent_record_offset":23652,"ancestor_record_offset":6424,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":7980,"parent_record_offset":7820,"ancestor_record_offset":6424,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":9908,"parent_record_offset":9732,"ancestor_record_offset":6424,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":12696,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":8532,"parent_record_offset":8504,"ancestor_record_offset":6424,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":22020,"parent_record_offset":21744,"ancestor_record_offset":6424,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":11324,"parent_record_offset":11300,"ancestor_record_offset":6424,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":6700,"parent_record_offset":6636,"ancestor_record_offset":6424,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":6424,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":6424,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":6424,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":14148,"parent_record_offset":14124,"ancestor_record_offset":6424,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":6424,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":7716,"parent_record_offset":7684,"ancestor_record_offset":6424,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":6424,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":6424,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":6424,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":7888,"parent_record_offset":7844,"ancestor_record_offset":6424,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":18504,"parent_record_offset":18476,"ancestor_record_offset":6424,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":11168,"parent_record_offset":11140,"ancestor_record_offset":6424,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":6424,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":39,"record_offset":10636,"parent_record_offset":9688,"ancestor_record_offset":6424,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":39,"record_offset":17936,"parent_record_offset":17852,"ancestor_record_offset":6424,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":39,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":6424,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":39,"record_offset":12724,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":8172,"parent_record_offset":8140,"ancestor_record_offset":6424,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":7248,"parent_record_offset":7096,"ancestor_record_offset":6424,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":39,"record_offset":8196,"parent_record_offset":8172,"ancestor_record_offset":6424,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":39,"record_offset":15760,"parent_record_offset":15728,"ancestor_record_offset":6424,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":39,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":6424,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":7920,"parent_record_offset":7844,"ancestor_record_offset":6424,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":8912,"parent_record_offset":8888,"ancestor_record_offset":6424,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":6424,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":11460,"parent_record_offset":11436,"ancestor_record_offset":6424,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":6424,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":6424,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":8888,"parent_record_offset":8780,"ancestor_record_offset":6424,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":39,"record_offset":8140,"parent_record_offset":8116,"ancestor_record_offset":6424,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":39,"record_offset":8620,"parent_record_offset":8584,"ancestor_record_offset":6424,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":39,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":6424,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":39,"record_offset":8724,"parent_record_offset":8692,"ancestor_record_offset":6424,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":39,"record_offset":8668,"parent_record_offset":8644,"ancestor_record_offset":6424,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":39,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":6424,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":39,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":6424,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":39,"record_offset":9216,"parent_record_offset":8692,"ancestor_record_offset":6424,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8332,"parent_record_offset":8300,"ancestor_record_offset":6424,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8480,"parent_record_offset":8332,"ancestor_record_offset":6424,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":6424,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8056,"parent_record_offset":8012,"ancestor_record_offset":6424,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":6424,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8116,"parent_record_offset":8084,"ancestor_record_offset":6424,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8084,"parent_record_offset":8056,"ancestor_record_offset":6424,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":39,"record_offset":8780,"parent_record_offset":8752,"ancestor_record_offset":6424,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":39,"record_offset":9028,"parent_record_offset":8780,"ancestor_record_offset":6424,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":39,"record_offset":9000,"parent_record_offset":8780,"ancestor_record_offset":6424,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":39,"record_offset":8832,"parent_record_offset":8780,"ancestor_record_offset":6424,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":18168,"parent_record_offset":18136,"ancestor_record_offset":6424,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":17880,"parent_record_offset":17852,"ancestor_record_offset":6424,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":18192,"parent_record_offset":18168,"ancestor_record_offset":6424,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":17904,"parent_record_offset":17880,"ancestor_record_offset":6424,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":7744,"parent_record_offset":7684,"ancestor_record_offset":6424,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":7776,"parent_record_offset":7648,"ancestor_record_offset":6424,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":39,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":6424,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":39,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":6424,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":39,"record_offset":16056,"parent_record_offset":16032,"ancestor_record_offset":6424,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] ecolor::color32::Color32::from_rgba_premultiplied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ecolor","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ecolor-0.27.2/src/color32.rs
//     evidence={"module":39,"record_offset":11568,"parent_record_offset":11512,"ancestor_record_offset":6424,"inlinee":6122,"source_line":89,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":39,"record_offset":17672,"parent_record_offset":17640,"ancestor_record_offset":6424,"inlinee":5798,"source_line":795,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":39,"record_offset":17704,"parent_record_offset":17672,"ancestor_record_offset":6424,"inlinee":5785,"source_line":717,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::impl$13::input::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":39,"record_offset":18060,"parent_record_offset":17704,"ancestor_record_offset":6424,"inlinee":5800,"source_line":796,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::ImageData::height
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":10764,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":7315,"source_line":33,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::ImageData::size
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":10716,"parent_record_offset":10676,"ancestor_record_offset":6424,"inlinee":7314,"source_line":22,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::ImageData::width
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":10676,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":7313,"source_line":29,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::fast_round
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":11540,"parent_record_offset":11512,"ancestor_record_offset":6424,"inlinee":6120,"source_line":359,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::impl$7::srgba_pixels::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":11512,"parent_record_offset":11488,"ancestor_record_offset":6424,"inlinee":6117,"source_line":305,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::impl$5::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/lib.rs
//     evidence={"module":39,"record_offset":12432,"parent_record_offset":12408,"ancestor_record_offset":6424,"inlinee":7321,"source_line":86,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":39,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":6424,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":14484,"parent_record_offset":14460,"ancestor_record_offset":6424,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":6424,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":6424,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":24252,"parent_record_offset":24224,"ancestor_record_offset":6424,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":12600,"parent_record_offset":11788,"ancestor_record_offset":6424,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":11788,"parent_record_offset":11756,"ancestor_record_offset":6424,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":15108,"parent_record_offset":14912,"ancestor_record_offset":6424,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":11828,"parent_record_offset":11788,"ancestor_record_offset":6424,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":19440,"parent_record_offset":19416,"ancestor_record_offset":6424,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":19416,"parent_record_offset":19304,"ancestor_record_offset":6424,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":6424,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":14124,"parent_record_offset":13180,"ancestor_record_offset":6424,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12560,"parent_record_offset":11880,"ancestor_record_offset":6424,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":6424,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":11856,"parent_record_offset":11788,"ancestor_record_offset":6424,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":18888,"parent_record_offset":18828,"ancestor_record_offset":6424,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13180,"parent_record_offset":13156,"ancestor_record_offset":6424,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12624,"parent_record_offset":12600,"ancestor_record_offset":6424,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13984,"parent_record_offset":13180,"ancestor_record_offset":6424,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":6424,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13252,"parent_record_offset":13220,"ancestor_record_offset":6424,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13432,"parent_record_offset":13252,"ancestor_record_offset":6424,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13596,"parent_record_offset":13252,"ancestor_record_offset":6424,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13628,"parent_record_offset":13596,"ancestor_record_offset":6424,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13220,"parent_record_offset":13180,"ancestor_record_offset":6424,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":11956,"parent_record_offset":11880,"ancestor_record_offset":6424,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":6424,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13888,"parent_record_offset":13220,"ancestor_record_offset":6424,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":39,"record_offset":12152,"parent_record_offset":12124,"ancestor_record_offset":6424,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":39,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":6424,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":39,"record_offset":12240,"parent_record_offset":12124,"ancestor_record_offset":6424,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":39,"record_offset":12124,"parent_record_offset":11880,"ancestor_record_offset":6424,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":11928,"parent_record_offset":11880,"ancestor_record_offset":6424,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12272,"parent_record_offset":11880,"ancestor_record_offset":6424,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":39,"record_offset":15020,"parent_record_offset":14936,"ancestor_record_offset":6424,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":39,"record_offset":14964,"parent_record_offset":14936,"ancestor_record_offset":6424,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":39,"record_offset":15364,"parent_record_offset":14584,"ancestor_record_offset":6424,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":39,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":6424,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":39,"record_offset":14560,"parent_record_offset":14536,"ancestor_record_offset":6424,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":39,"record_offset":14536,"parent_record_offset":14508,"ancestor_record_offset":6424,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":39,"record_offset":14584,"parent_record_offset":14560,"ancestor_record_offset":6424,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":39,"record_offset":14792,"parent_record_offset":14764,"ancestor_record_offset":6424,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":39,"record_offset":14820,"parent_record_offset":14792,"ancestor_record_offset":6424,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":39,"record_offset":14880,"parent_record_offset":14764,"ancestor_record_offset":6424,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":39,"record_offset":14764,"parent_record_offset":14584,"ancestor_record_offset":6424,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":39,"record_offset":14912,"parent_record_offset":14584,"ancestor_record_offset":6424,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":39,"record_offset":14624,"parent_record_offset":14584,"ancestor_record_offset":6424,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":39,"record_offset":14708,"parent_record_offset":14584,"ancestor_record_offset":6424,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":39,"record_offset":15280,"parent_record_offset":14584,"ancestor_record_offset":6424,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13652,"parent_record_offset":13628,"ancestor_record_offset":6424,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":14096,"parent_record_offset":13180,"ancestor_record_offset":6424,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":11984,"parent_record_offset":11880,"ancestor_record_offset":6424,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":13684,"parent_record_offset":13596,"ancestor_record_offset":6424,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":12068,"parent_record_offset":11880,"ancestor_record_offset":6424,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":12468,"parent_record_offset":11880,"ancestor_record_offset":6424,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":13472,"parent_record_offset":13432,"ancestor_record_offset":6424,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":13156,"parent_record_offset":13132,"ancestor_record_offset":6424,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":12408,"parent_record_offset":12272,"ancestor_record_offset":6424,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":39,"record_offset":17828,"parent_record_offset":17740,"ancestor_record_offset":6424,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLockWriteGuard::map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":39,"record_offset":18000,"parent_record_offset":17740,"ancestor_record_offset":6424,"inlinee":5789,"source_line":1494,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$52::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":39,"record_offset":18112,"parent_record_offset":18088,"ancestor_record_offset":6424,"inlinee":5794,"source_line":2576,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":39,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":6424,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":39,"record_offset":15728,"parent_record_offset":15704,"ancestor_record_offset":6424,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":39,"record_offset":17852,"parent_record_offset":17828,"ancestor_record_offset":6424,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":39,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":6424,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":9600,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":9644,"parent_record_offset":9600,"ancestor_record_offset":6424,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":8012,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":39,"record_offset":9772,"parent_record_offset":9732,"ancestor_record_offset":6424,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":9936,"parent_record_offset":9732,"ancestor_record_offset":6424,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":10208,"parent_record_offset":9936,"ancestor_record_offset":6424,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":8584,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":39,"record_offset":12668,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":39,"record_offset":22424,"parent_record_offset":22400,"ancestor_record_offset":6424,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":39,"record_offset":22340,"parent_record_offset":22316,"ancestor_record_offset":6424,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":11756,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":24200,"parent_record_offset":23652,"ancestor_record_offset":6424,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":13132,"parent_record_offset":6904,"ancestor_record_offset":6424,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":39,"record_offset":9800,"parent_record_offset":9772,"ancestor_record_offset":6424,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":39,"record_offset":9832,"parent_record_offset":9800,"ancestor_record_offset":6424,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":39,"record_offset":9732,"parent_record_offset":9688,"ancestor_record_offset":6424,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":39,"record_offset":9688,"parent_record_offset":9644,"ancestor_record_offset":6424,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::update_textures
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C73
// 首条位置: {"module":39,"inlinee":7283}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10796,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":5202,"source_line":1227,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":7096,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":39,"record_offset":7820,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":10824,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":12696,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":7336,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":39,"record_offset":12724,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] epaint::image::ImageData::height
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":10764,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":7315,"source_line":33,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] epaint::image::ImageData::width
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":10676,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":7313,"source_line":29,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":9600,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":8012,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":8584,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":39,"record_offset":12668,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":11756,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":13132,"parent_record_offset":6904,"ancestor_record_offset":6904,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":11116,"parent_record_offset":11092,"ancestor_record_offset":6904,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":11140,"parent_record_offset":11116,"ancestor_record_offset":6904,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":7576,"parent_record_offset":7552,"ancestor_record_offset":6904,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":11092,"parent_record_offset":11056,"ancestor_record_offset":6904,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":7552,"parent_record_offset":7464,"ancestor_record_offset":6904,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":7492,"parent_record_offset":7464,"ancestor_record_offset":6904,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":6904,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":11056,"parent_record_offset":11024,"ancestor_record_offset":6904,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":11024,"parent_record_offset":10992,"ancestor_record_offset":6904,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":11212,"parent_record_offset":11024,"ancestor_record_offset":6904,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":7464,"parent_record_offset":7440,"ancestor_record_offset":6904,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":39,"record_offset":7684,"parent_record_offset":7648,"ancestor_record_offset":6904,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":39,"record_offset":7648,"parent_record_offset":7624,"ancestor_record_offset":6904,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":7136,"parent_record_offset":7096,"ancestor_record_offset":6904,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":11388,"parent_record_offset":11364,"ancestor_record_offset":6904,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":7192,"parent_record_offset":7096,"ancestor_record_offset":6904,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":7220,"parent_record_offset":7096,"ancestor_record_offset":6904,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10960,"parent_record_offset":10920,"ancestor_record_offset":6904,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10992,"parent_record_offset":10960,"ancestor_record_offset":6904,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":10856,"parent_record_offset":10824,"ancestor_record_offset":6904,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":6904,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":39,"record_offset":11680,"parent_record_offset":11624,"ancestor_record_offset":6904,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":39,"record_offset":11364,"parent_record_offset":10920,"ancestor_record_offset":6904,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":39,"record_offset":10888,"parent_record_offset":10856,"ancestor_record_offset":6904,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":39,"record_offset":10920,"parent_record_offset":10888,"ancestor_record_offset":6904,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10028,"parent_record_offset":10000,"ancestor_record_offset":6904,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10080,"parent_record_offset":10056,"ancestor_record_offset":6904,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10056,"parent_record_offset":10028,"ancestor_record_offset":6904,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":9968,"parent_record_offset":9936,"ancestor_record_offset":6904,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10000,"parent_record_offset":9968,"ancestor_record_offset":6904,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10172,"parent_record_offset":10000,"ancestor_record_offset":6904,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":39,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":6904,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":13708,"parent_record_offset":13684,"ancestor_record_offset":6904,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":6904,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":39,"record_offset":12092,"parent_record_offset":12068,"ancestor_record_offset":6904,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":39,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":6904,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":39,"record_offset":11436,"parent_record_offset":11412,"ancestor_record_offset":6904,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":39,"record_offset":11252,"parent_record_offset":10920,"ancestor_record_offset":6904,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":39,"record_offset":11488,"parent_record_offset":11460,"ancestor_record_offset":6904,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":11412,"parent_record_offset":11388,"ancestor_record_offset":6904,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":11600,"parent_record_offset":11488,"ancestor_record_offset":6904,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":39,"record_offset":10104,"parent_record_offset":10080,"ancestor_record_offset":6904,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":6904,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":6904,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":39,"record_offset":8228,"parent_record_offset":8140,"ancestor_record_offset":6904,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":39,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":6904,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":9860,"parent_record_offset":9832,"ancestor_record_offset":6904,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":7980,"parent_record_offset":7820,"ancestor_record_offset":6904,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":9908,"parent_record_offset":9732,"ancestor_record_offset":6904,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":8532,"parent_record_offset":8504,"ancestor_record_offset":6904,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":11324,"parent_record_offset":11300,"ancestor_record_offset":6904,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":7364,"parent_record_offset":7336,"ancestor_record_offset":6904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":6904,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":6904,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":6904,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":14148,"parent_record_offset":14124,"ancestor_record_offset":6904,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":6904,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":7716,"parent_record_offset":7684,"ancestor_record_offset":6904,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":6904,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":6904,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":6904,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":7888,"parent_record_offset":7844,"ancestor_record_offset":6904,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":11168,"parent_record_offset":11140,"ancestor_record_offset":6904,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":6904,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":39,"record_offset":10636,"parent_record_offset":9688,"ancestor_record_offset":6904,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":8172,"parent_record_offset":8140,"ancestor_record_offset":6904,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":7248,"parent_record_offset":7096,"ancestor_record_offset":6904,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":39,"record_offset":8196,"parent_record_offset":8172,"ancestor_record_offset":6904,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":39,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":6904,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":7920,"parent_record_offset":7844,"ancestor_record_offset":6904,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":8912,"parent_record_offset":8888,"ancestor_record_offset":6904,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":6904,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":11460,"parent_record_offset":11436,"ancestor_record_offset":6904,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":6904,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":6904,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":8888,"parent_record_offset":8780,"ancestor_record_offset":6904,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":39,"record_offset":8140,"parent_record_offset":8116,"ancestor_record_offset":6904,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":39,"record_offset":8620,"parent_record_offset":8584,"ancestor_record_offset":6904,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":39,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":6904,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":39,"record_offset":8724,"parent_record_offset":8692,"ancestor_record_offset":6904,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":39,"record_offset":8668,"parent_record_offset":8644,"ancestor_record_offset":6904,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":39,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":6904,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":39,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":6904,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":39,"record_offset":9216,"parent_record_offset":8692,"ancestor_record_offset":6904,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8332,"parent_record_offset":8300,"ancestor_record_offset":6904,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8480,"parent_record_offset":8332,"ancestor_record_offset":6904,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":6904,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8056,"parent_record_offset":8012,"ancestor_record_offset":6904,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":6904,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8116,"parent_record_offset":8084,"ancestor_record_offset":6904,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":39,"record_offset":8084,"parent_record_offset":8056,"ancestor_record_offset":6904,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":39,"record_offset":8780,"parent_record_offset":8752,"ancestor_record_offset":6904,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":39,"record_offset":9028,"parent_record_offset":8780,"ancestor_record_offset":6904,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":39,"record_offset":9000,"parent_record_offset":8780,"ancestor_record_offset":6904,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":39,"record_offset":8832,"parent_record_offset":8780,"ancestor_record_offset":6904,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":7744,"parent_record_offset":7684,"ancestor_record_offset":6904,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":39,"record_offset":7776,"parent_record_offset":7648,"ancestor_record_offset":6904,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] ecolor::color32::Color32::from_rgba_premultiplied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ecolor","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ecolor-0.27.2/src/color32.rs
//     evidence={"module":39,"record_offset":11568,"parent_record_offset":11512,"ancestor_record_offset":6904,"inlinee":6122,"source_line":89,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::ImageData::size
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":10716,"parent_record_offset":10676,"ancestor_record_offset":6904,"inlinee":7314,"source_line":22,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::fast_round
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":11540,"parent_record_offset":11512,"ancestor_record_offset":6904,"inlinee":6120,"source_line":359,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::impl$7::srgba_pixels::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/image.rs
//     evidence={"module":39,"record_offset":11512,"parent_record_offset":11488,"ancestor_record_offset":6904,"inlinee":6117,"source_line":305,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] epaint::impl$5::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.27.2/src/lib.rs
//     evidence={"module":39,"record_offset":12432,"parent_record_offset":12408,"ancestor_record_offset":6904,"inlinee":7321,"source_line":86,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":12600,"parent_record_offset":11788,"ancestor_record_offset":6904,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":11788,"parent_record_offset":11756,"ancestor_record_offset":6904,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":11828,"parent_record_offset":11788,"ancestor_record_offset":6904,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":6904,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":14124,"parent_record_offset":13180,"ancestor_record_offset":6904,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12560,"parent_record_offset":11880,"ancestor_record_offset":6904,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":6904,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":11856,"parent_record_offset":11788,"ancestor_record_offset":6904,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13180,"parent_record_offset":13156,"ancestor_record_offset":6904,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12624,"parent_record_offset":12600,"ancestor_record_offset":6904,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13984,"parent_record_offset":13180,"ancestor_record_offset":6904,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":6904,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13252,"parent_record_offset":13220,"ancestor_record_offset":6904,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13432,"parent_record_offset":13252,"ancestor_record_offset":6904,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13596,"parent_record_offset":13252,"ancestor_record_offset":6904,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13628,"parent_record_offset":13596,"ancestor_record_offset":6904,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13220,"parent_record_offset":13180,"ancestor_record_offset":6904,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":11956,"parent_record_offset":11880,"ancestor_record_offset":6904,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":6904,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13888,"parent_record_offset":13220,"ancestor_record_offset":6904,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":39,"record_offset":12152,"parent_record_offset":12124,"ancestor_record_offset":6904,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":39,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":6904,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":39,"record_offset":12240,"parent_record_offset":12124,"ancestor_record_offset":6904,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":39,"record_offset":12124,"parent_record_offset":11880,"ancestor_record_offset":6904,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":11928,"parent_record_offset":11880,"ancestor_record_offset":6904,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":12272,"parent_record_offset":11880,"ancestor_record_offset":6904,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":13652,"parent_record_offset":13628,"ancestor_record_offset":6904,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":39,"record_offset":14096,"parent_record_offset":13180,"ancestor_record_offset":6904,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":11984,"parent_record_offset":11880,"ancestor_record_offset":6904,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":13684,"parent_record_offset":13596,"ancestor_record_offset":6904,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":12068,"parent_record_offset":11880,"ancestor_record_offset":6904,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":12468,"parent_record_offset":11880,"ancestor_record_offset":6904,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":39,"record_offset":13472,"parent_record_offset":13432,"ancestor_record_offset":6904,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":13156,"parent_record_offset":13132,"ancestor_record_offset":6904,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":39,"record_offset":12408,"parent_record_offset":12272,"ancestor_record_offset":6904,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":9644,"parent_record_offset":9600,"ancestor_record_offset":6904,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":39,"record_offset":9772,"parent_record_offset":9732,"ancestor_record_offset":6904,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":9936,"parent_record_offset":9732,"ancestor_record_offset":6904,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":39,"record_offset":10208,"parent_record_offset":9936,"ancestor_record_offset":6904,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":39,"record_offset":9800,"parent_record_offset":9772,"ancestor_record_offset":6904,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":39,"record_offset":9832,"parent_record_offset":9800,"ancestor_record_offset":6904,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":39,"record_offset":9732,"parent_record_offset":9688,"ancestor_record_offset":6904,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":39,"record_offset":9688,"parent_record_offset":9644,"ancestor_record_offset":6904,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::release_pending_textures
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CAE
// 首条位置: {"module":39,"inlinee":7342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23704,"parent_record_offset":23652,"ancestor_record_offset":23652,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":39,"record_offset":23944,"parent_record_offset":23652,"ancestor_record_offset":23652,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":24320,"parent_record_offset":23652,"ancestor_record_offset":23652,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":24360,"parent_record_offset":23652,"ancestor_record_offset":23652,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":39,"record_offset":24200,"parent_record_offset":23652,"ancestor_record_offset":23652,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":39,"record_offset":23768,"parent_record_offset":23744,"ancestor_record_offset":23652,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23744,"parent_record_offset":23704,"ancestor_record_offset":23652,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23800,"parent_record_offset":23704,"ancestor_record_offset":23652,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":23828,"parent_record_offset":23704,"ancestor_record_offset":23652,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":39,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":23652,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":39,"record_offset":24112,"parent_record_offset":23944,"ancestor_record_offset":23652,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":24388,"parent_record_offset":24360,"ancestor_record_offset":23652,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":23904,"parent_record_offset":23880,"ancestor_record_offset":23652,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":24076,"parent_record_offset":24052,"ancestor_record_offset":23652,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":24020,"parent_record_offset":23972,"ancestor_record_offset":23652,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":24160,"parent_record_offset":24136,"ancestor_record_offset":23652,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":39,"record_offset":23856,"parent_record_offset":23704,"ancestor_record_offset":23652,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":39,"record_offset":23880,"parent_record_offset":23856,"ancestor_record_offset":23652,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":24052,"parent_record_offset":23972,"ancestor_record_offset":23652,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":23972,"parent_record_offset":23944,"ancestor_record_offset":23652,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":24224,"parent_record_offset":24200,"ancestor_record_offset":23652,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":24252,"parent_record_offset":24224,"ancestor_record_offset":23652,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":39,"record_offset":24280,"parent_record_offset":24252,"ancestor_record_offset":23652,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00039.jsonl.gz"}
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
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: egui_backend.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::draw
// 信息 ID: f4b5dcf46995bce5d6ea2fa9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffd7a -> 0x5c230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::end_frame
// 信息 ID: 8993f8af798e007c9a908dbe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24bcef -> 0x5bee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::new
// 信息 ID: 2c2dcb58ad5b0bb8ad67c3b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=58400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::release_pending_textures
// 信息 ID: 7f99ea5d66a0cafe3c483770
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=23652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::take_input
// 信息 ID: 0d9e65866459f3b6a322b841
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=113856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::update_textures
// 信息 ID: 91b513d440b7b5c923c70dac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=6904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
