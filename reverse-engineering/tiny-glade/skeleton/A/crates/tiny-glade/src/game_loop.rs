// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/game_loop.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fd24d2d28151f47d24699fdff243448960fd3394283f991f14116fc0a59a3d33
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: game_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 21, 127, 139, 256
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_input; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"PDB_registry_path_only","version":"1.19.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use smol_str; // namespace candidate
//   package identities: [{"name":"smol_str","source":"PDB_registry_path_only","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// use winit; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.4.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::new
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x29E1
// 首条位置: {"module":256,"inlinee":10721}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":127,"record_offset":8020,"rva":2383280,"va":5371092400}
// 编译实例 RVAs: 0x245db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14019f730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-682142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e86297f5ff","size":5,"soft_fail":false,"target_va":5370410800,"va":5371092937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036f090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1215748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8048d1200","size":5,"soft_fail":false,"target_va":5372309648,"va":5371093895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1221126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e806a21200","size":5,"soft_fail":false,"target_va":5372314208,"va":5371093077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1225850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e87ab41200","size":5,"soft_fail":false,"target_va":5372320512,"va":5371094657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371e50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1227403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e88bba1200","size":5,"soft_fail":false,"target_va":5372321360,"va":5371093952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037d2f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1273545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8c96e1300","size":5,"soft_fail":false,"target_va":5372367600,"va":5371094050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fe40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1284796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8bc9a1300","size":5,"soft_fail":false,"target_va":5372378688,"va":5371093887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038c0a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1334536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8085d1400","size":5,"soft_fail":false,"target_va":5372428448,"va":5371093907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140392b60
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1361803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e88bc71400","size":5,"soft_fail":false,"target_va":5372455776,"va":5371093968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1656814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8ee471900","size":5,"soft_fail":false,"target_va":5372749696,"va":5371092877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1656654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e84e471900","size":5,"soft_fail":false,"target_va":5372749712,"va":5371093053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4078049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8e1393e00","size":5,"soft_fail":false,"target_va":5375171056,"va":5371093002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4279010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8e24a4100","size":5,"soft_fail":false,"target_va":5375372576,"va":5371093561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5145928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e848854e00","size":5,"soft_fail":false,"target_va":5376239920,"va":5371093987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32500847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e86fecef01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371092508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31883855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e84f82e601","size":5,"soft_fail":false,"target_va":5402978144,"va":5371094284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t558492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e89c850800","size":5,"soft_fail":false,"target_va":5371652368,"va":5371093871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6271849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e869b35f00","size":5,"soft_fail":false,"target_va":5377364336,"va":5371092482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31884622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e84e85e601","size":5,"soft_fail":false,"target_va":5402978880,"va":5371094253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31885743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8af89e601","size":5,"soft_fail":false,"target_va":5402980128,"va":5371094380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6259925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8d5845f00","size":5,"soft_fail":false,"target_va":5377353776,"va":5371093846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31875390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e83e61e601","size":5,"soft_fail":false,"target_va":5402968992,"va":5371093597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31877141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e81568e601","size":5,"soft_fail":false,"target_va":5402970800,"va":5371093654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5131899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e87b4e4e00","size":5,"soft_fail":false,"target_va":5376225520,"va":5371093616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::peak_system_bytes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5131904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8804e4e00","size":5,"soft_fail":false,"target_va":5376225536,"va":5371093627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6313279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e83f556000","size":5,"soft_fail":false,"target_va":5377406304,"va":5371093020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::game_loop::GameLoop::main_loop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8090c0000","size":5,"soft_fail":false,"target_va":5371095536,"va":5371092450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_fullscreen
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371092400,"raw_hex":"e8daf6ffff","size":5,"soft_fail":false,"target_va":5371090208,"va":5371092545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":8144,"parent_record_offset":8020,"ancestor_record_offset":8020,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::shut_down_everything
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":8216,"parent_record_offset":8020,"ancestor_record_offset":8020,"inlinee":10618,"source_line":879,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":8020,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":8020,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":8020,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10104,"parent_record_offset":10076,"ancestor_record_offset":8020,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10488,"parent_record_offset":10432,"ancestor_record_offset":8020,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10460,"parent_record_offset":10432,"ancestor_record_offset":8020,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10076,"parent_record_offset":10048,"ancestor_record_offset":8020,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10048,"parent_record_offset":10020,"ancestor_record_offset":8020,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10224,"parent_record_offset":10048,"ancestor_record_offset":8020,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10432,"parent_record_offset":10404,"ancestor_record_offset":8020,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":14368,"parent_record_offset":14332,"ancestor_record_offset":8020,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":8316,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":14332,"parent_record_offset":14308,"ancestor_record_offset":8020,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":9992,"parent_record_offset":9944,"ancestor_record_offset":8020,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":10020,"parent_record_offset":9992,"ancestor_record_offset":8020,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":8404,"parent_record_offset":8372,"ancestor_record_offset":8020,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":9860,"parent_record_offset":9828,"ancestor_record_offset":8020,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":12940,"parent_record_offset":12848,"ancestor_record_offset":8020,"inlinee":10627,"source_line":294,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":9804,"parent_record_offset":9416,"ancestor_record_offset":8020,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":8020,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":12848,"parent_record_offset":11432,"ancestor_record_offset":8020,"inlinee":10626,"source_line":210,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":9444,"parent_record_offset":9416,"ancestor_record_offset":8020,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":12476,"parent_record_offset":11432,"ancestor_record_offset":8020,"inlinee":10622,"source_line":297,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":9492,"parent_record_offset":9468,"ancestor_record_offset":8020,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":8020,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":12512,"parent_record_offset":12476,"ancestor_record_offset":8020,"inlinee":10623,"source_line":379,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":9584,"parent_record_offset":9520,"ancestor_record_offset":8020,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":9672,"parent_record_offset":9648,"ancestor_record_offset":8020,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":12716,"parent_record_offset":12692,"ancestor_record_offset":8020,"inlinee":10625,"source_line":381,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":8344,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::remove_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":11432,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":10621,"source_line":1365,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":8372,"parent_record_offset":8344,"ancestor_record_offset":8020,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":9416,"parent_record_offset":8372,"ancestor_record_offset":8020,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":127,"record_offset":13356,"parent_record_offset":11432,"ancestor_record_offset":8020,"inlinee":10628,"source_line":428,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":9148,"parent_record_offset":9124,"ancestor_record_offset":8020,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":8020,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":14200,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":8020,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":8020,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":127,"record_offset":10776,"parent_record_offset":10740,"ancestor_record_offset":8020,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":8020,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":8020,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":127,"record_offset":14228,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":8020,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":13084,"parent_record_offset":13012,"ancestor_record_offset":8020,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":8020,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":8020,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":9384,"parent_record_offset":8404,"ancestor_record_offset":8020,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":9648,"parent_record_offset":9468,"ancestor_record_offset":8020,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":14500,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":8020,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":8164,"parent_record_offset":8144,"ancestor_record_offset":8020,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":8020,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":8020,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":13380,"parent_record_offset":13356,"ancestor_record_offset":8020,"inlinee":6972,"source_line":1335,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":8020,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":14400,"parent_record_offset":14368,"ancestor_record_offset":8020,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":13160,"parent_record_offset":13084,"ancestor_record_offset":8020,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":8020,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":12980,"parent_record_offset":12940,"ancestor_record_offset":8020,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":13012,"parent_record_offset":12980,"ancestor_record_offset":8020,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":13248,"parent_record_offset":13084,"ancestor_record_offset":8020,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":13612,"parent_record_offset":13528,"ancestor_record_offset":8020,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":13636,"parent_record_offset":13612,"ancestor_record_offset":8020,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":14556,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":9520,"parent_record_offset":9492,"ancestor_record_offset":8020,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":9556,"parent_record_offset":9520,"ancestor_record_offset":8020,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":9728,"parent_record_offset":9696,"ancestor_record_offset":8020,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":12772,"parent_record_offset":12740,"ancestor_record_offset":8020,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13840,"parent_record_offset":13808,"ancestor_record_offset":8020,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13556,"parent_record_offset":13528,"ancestor_record_offset":8020,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":15044,"parent_record_offset":15008,"ancestor_record_offset":8020,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":8020,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":8020,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":8020,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":8020,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":14428,"parent_record_offset":14368,"ancestor_record_offset":8020,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":14460,"parent_record_offset":14332,"ancestor_record_offset":8020,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":8020,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":8020,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":8432,"parent_record_offset":8404,"ancestor_record_offset":8020,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":8020,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":9076,"parent_record_offset":8876,"ancestor_record_offset":8020,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8988,"parent_record_offset":8904,"ancestor_record_offset":8020,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8932,"parent_record_offset":8904,"ancestor_record_offset":8020,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":9332,"parent_record_offset":8540,"ancestor_record_offset":8020,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8904,"parent_record_offset":8876,"ancestor_record_offset":8020,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":8020,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8492,"parent_record_offset":8460,"ancestor_record_offset":8020,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":8020,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":8756,"parent_record_offset":8728,"ancestor_record_offset":8020,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":8784,"parent_record_offset":8756,"ancestor_record_offset":8020,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":8844,"parent_record_offset":8728,"ancestor_record_offset":8020,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":8728,"parent_record_offset":8540,"ancestor_record_offset":8020,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8876,"parent_record_offset":8540,"ancestor_record_offset":8020,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":8588,"parent_record_offset":8540,"ancestor_record_offset":8020,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":8672,"parent_record_offset":8540,"ancestor_record_offset":8020,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":9248,"parent_record_offset":8540,"ancestor_record_offset":8020,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13504,"parent_record_offset":13444,"ancestor_record_offset":8020,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13676,"parent_record_offset":13444,"ancestor_record_offset":8020,"inlinee":10631,"source_line":639,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":8020,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":127,"record_offset":14256,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":127,"record_offset":14116,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":8020,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":127,"record_offset":13528,"parent_record_offset":13504,"ancestor_record_offset":8020,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":127,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":8020,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::with_global_allocator
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":127,"record_offset":13444,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":10629,"source_line":684,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":127,"record_offset":10348,"parent_record_offset":10264,"ancestor_record_offset":8020,"inlinee":5032,"source_line":499,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":10292,"parent_record_offset":10264,"ancestor_record_offset":8020,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":8020,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":9908,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":10619,"source_line":2217,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":9944,"parent_record_offset":9908,"ancestor_record_offset":8020,"inlinee":10184,"source_line":2185,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":10264,"parent_record_offset":9944,"ancestor_record_offset":8020,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":127,"record_offset":10888,"parent_record_offset":10860,"ancestor_record_offset":8020,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":127,"record_offset":10944,"parent_record_offset":10916,"ancestor_record_offset":8020,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":127,"record_offset":10916,"parent_record_offset":10888,"ancestor_record_offset":8020,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":127,"record_offset":10740,"parent_record_offset":8216,"ancestor_record_offset":8020,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":127,"record_offset":10860,"parent_record_offset":10832,"ancestor_record_offset":8020,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":127,"record_offset":10832,"parent_record_offset":10804,"ancestor_record_offset":8020,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":127,"record_offset":10804,"parent_record_offset":10776,"ancestor_record_offset":8020,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::shut_down_everything::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":13732,"parent_record_offset":13444,"ancestor_record_offset":8020,"inlinee":10633,"source_line":893,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::receive_app_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x298F
// 首条位置: {"module":127,"inlinee":10639}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":47988,"parent_record_offset":47936,"ancestor_record_offset":47936,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":127,"record_offset":48100,"parent_record_offset":47936,"ancestor_record_offset":47936,"inlinee":7942,"source_line":789,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":48040,"parent_record_offset":47988,"ancestor_record_offset":47936,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":48068,"parent_record_offset":47988,"ancestor_record_offset":47936,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":127,"record_offset":48124,"parent_record_offset":48100,"ancestor_record_offset":47936,"inlinee":7892,"source_line":287,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::send_window_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2990
// 首条位置: {"module":127,"inlinee":10640}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":50444,"parent_record_offset":48172,"ancestor_record_offset":48172,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":127,"record_offset":50412,"parent_record_offset":48172,"ancestor_record_offset":48172,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":48280,"parent_record_offset":48172,"ancestor_record_offset":48172,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":49816,"parent_record_offset":48172,"ancestor_record_offset":48172,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":127,"record_offset":49788,"parent_record_offset":48172,"ancestor_record_offset":48172,"inlinee":5570,"source_line":612,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::next_request_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":49760,"parent_record_offset":48172,"ancestor_record_offset":48172,"inlinee":10642,"source_line":935,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":50108,"parent_record_offset":50080,"ancestor_record_offset":48172,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":50080,"parent_record_offset":50052,"ancestor_record_offset":48172,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":50292,"parent_record_offset":50256,"ancestor_record_offset":48172,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":50052,"parent_record_offset":50028,"ancestor_record_offset":48172,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":48332,"parent_record_offset":48304,"ancestor_record_offset":48172,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":49712,"parent_record_offset":49680,"ancestor_record_offset":48172,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":49656,"parent_record_offset":49332,"ancestor_record_offset":48172,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":49680,"parent_record_offset":49656,"ancestor_record_offset":48172,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":49360,"parent_record_offset":49332,"ancestor_record_offset":48172,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":49408,"parent_record_offset":49384,"ancestor_record_offset":48172,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":49384,"parent_record_offset":49360,"ancestor_record_offset":48172,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":49500,"parent_record_offset":49436,"ancestor_record_offset":48172,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":49588,"parent_record_offset":49564,"ancestor_record_offset":48172,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":48304,"parent_record_offset":48280,"ancestor_record_offset":48172,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":49332,"parent_record_offset":48304,"ancestor_record_offset":48172,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":49064,"parent_record_offset":49040,"ancestor_record_offset":48172,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":49112,"parent_record_offset":49088,"ancestor_record_offset":48172,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":49040,"parent_record_offset":49016,"ancestor_record_offset":48172,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":48528,"parent_record_offset":48504,"ancestor_record_offset":48172,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":48612,"parent_record_offset":48588,"ancestor_record_offset":48172,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":48552,"parent_record_offset":48528,"ancestor_record_offset":48172,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":48724,"parent_record_offset":48700,"ancestor_record_offset":48172,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":49524,"parent_record_offset":49500,"ancestor_record_offset":48172,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":49300,"parent_record_offset":48332,"ancestor_record_offset":48172,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":49564,"parent_record_offset":49384,"ancestor_record_offset":48172,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":49844,"parent_record_offset":49816,"ancestor_record_offset":48172,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":48952,"parent_record_offset":48928,"ancestor_record_offset":48172,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":48872,"parent_record_offset":48848,"ancestor_record_offset":48172,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":50492,"parent_record_offset":50468,"ancestor_record_offset":48172,"inlinee":10644,"source_line":1220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":50468,"parent_record_offset":50444,"ancestor_record_offset":48172,"inlinee":10643,"source_line":1242,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":50516,"parent_record_offset":50492,"ancestor_record_offset":48172,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":51232,"parent_record_offset":51176,"ancestor_record_offset":48172,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":49436,"parent_record_offset":49408,"ancestor_record_offset":48172,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":49472,"parent_record_offset":49436,"ancestor_record_offset":48172,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":49088,"parent_record_offset":49064,"ancestor_record_offset":48172,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":49016,"parent_record_offset":48992,"ancestor_record_offset":48172,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":48356,"parent_record_offset":48332,"ancestor_record_offset":48172,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":48380,"parent_record_offset":48356,"ancestor_record_offset":48172,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":48992,"parent_record_offset":48792,"ancestor_record_offset":48172,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":48904,"parent_record_offset":48820,"ancestor_record_offset":48172,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":48848,"parent_record_offset":48820,"ancestor_record_offset":48172,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":49248,"parent_record_offset":48456,"ancestor_record_offset":48172,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":48820,"parent_record_offset":48792,"ancestor_record_offset":48172,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":48432,"parent_record_offset":48408,"ancestor_record_offset":48172,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":48408,"parent_record_offset":48380,"ancestor_record_offset":48172,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":48456,"parent_record_offset":48432,"ancestor_record_offset":48172,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":48672,"parent_record_offset":48644,"ancestor_record_offset":48172,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":48700,"parent_record_offset":48672,"ancestor_record_offset":48172,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":48760,"parent_record_offset":48644,"ancestor_record_offset":48172,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":48644,"parent_record_offset":48456,"ancestor_record_offset":48172,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":48792,"parent_record_offset":48456,"ancestor_record_offset":48172,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":48504,"parent_record_offset":48456,"ancestor_record_offset":48172,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":48588,"parent_record_offset":48456,"ancestor_record_offset":48172,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":49164,"parent_record_offset":48456,"ancestor_record_offset":48172,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":49612,"parent_record_offset":49588,"ancestor_record_offset":48172,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":127,"record_offset":51176,"parent_record_offset":51136,"ancestor_record_offset":48172,"inlinee":10646,"source_line":125,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$4::send::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":127,"record_offset":51272,"parent_record_offset":51232,"ancestor_record_offset":48172,"inlinee":10648,"source_line":131,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::main_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":127,"record_offset":15820,"rva":2386416,"va":5371095536}
// 编译实例 RVAs: 0x2469f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::try_fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t802077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e81d3d0c00","size":5,"soft_fail":false,"target_va":5371921424,"va":5371119342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$egui..data..input..RawInput$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5422702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e86ebe5200","size":5,"soft_fail":false,"target_va":5376537536,"va":5371114829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2156842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e82ae92000","size":5,"soft_fail":false,"target_va":5373265408,"va":5371108561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1487976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8984be9ff","size":5,"soft_fail":false,"target_va":5369632448,"va":5371120419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e1dd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1486310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e81a52e9ff","size":5,"soft_fail":false,"target_va":5369634256,"va":5371120561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e2ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1483089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8af5ee9ff","size":5,"soft_fail":false,"target_va":5369637552,"va":5371120636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1453044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e80cd4e9ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5371126783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ff9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1368414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8a21eebff","size":5,"soft_fail":false,"target_va":5369756144,"va":5371124553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140107320
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1344140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8747debff","size":5,"soft_fail":false,"target_va":5369787168,"va":5371131303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140174dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-865603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8bdcaf2ff","size":5,"soft_fail":false,"target_va":5370236352,"va":5371101950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140174f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-865756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e824caf2ff","size":5,"soft_fail":false,"target_va":5370236784,"va":5371102535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401751a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-864352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8a0cff2ff","size":5,"soft_fail":false,"target_va":5370237344,"va":5371101691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140286960
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t259092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e814f40300","size":5,"soft_fail":false,"target_va":5371357536,"va":5371098439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140289740
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t271363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e803240400","size":5,"soft_fail":false,"target_va":5371369280,"va":5371097912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14028d140
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t282986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e86a510400","size":5,"soft_fail":false,"target_va":5371384128,"va":5371101137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14028fbc0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t295392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8e0810400","size":5,"soft_fail":false,"target_va":5371395008,"va":5371099611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140291960
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t303561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c9a10400","size":5,"soft_fail":false,"target_va":5371402592,"va":5371099026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c10a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t484166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e846630700","size":5,"soft_fail":false,"target_va":5371596960,"va":5371112789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ceb70
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t543075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e863490800","size":5,"soft_fail":false,"target_va":5371652976,"va":5371109896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402eb4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t662816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8201d0a00","size":5,"soft_fail":false,"target_va":5371770016,"va":5371107195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030b3a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t783815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c7f50b00","size":5,"soft_fail":false,"target_va":5371900832,"va":5371117012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035dea0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1111237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c5f41000","size":5,"soft_fail":false,"target_va":5372239520,"va":5371128278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=2
//     target_aliases=79 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1120885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8751a1100","size":5,"soft_fail":false,"target_va":5372249392,"va":5371128502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361a60
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1125832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c82d1100","size":5,"soft_fail":false,"target_va":5372254816,"va":5371128979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1184943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8af141200","size":5,"soft_fail":false,"target_va":5372316880,"va":5371131932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371e50
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1224774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e846b01200","size":5,"soft_fail":false,"target_va":5372321360,"va":5371096581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140372de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1199516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e89c4d1200","size":5,"soft_fail":false,"target_va":5372325344,"va":5371125823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037aa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1226391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e897b61200","size":5,"soft_fail":false,"target_va":5372357248,"va":5371130852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037b180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1229255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c7c11200","size":5,"soft_fail":false,"target_va":5372359040,"va":5371129780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403841f0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1286040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8989f1300","size":5,"soft_fail":false,"target_va":5372396016,"va":5371109971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1287696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e810a61300","size":5,"soft_fail":false,"target_va":5372408976,"va":5371121275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140388cb0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1313223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c7091400","size":5,"soft_fail":false,"target_va":5372415152,"va":5371101924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038cc50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1310109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e89dfd1300","size":5,"soft_fail":false,"target_va":5372431440,"va":5371121326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1339268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8846f1400","size":5,"soft_fail":false,"target_va":5372449840,"va":5371110567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140394910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1337200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e870671400","size":5,"soft_fail":false,"target_va":5372463376,"va":5371126171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b8260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1508512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8a0041700","size":5,"soft_fail":false,"target_va":5372609120,"va":5371100603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1512500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e834141700","size":5,"soft_fail":false,"target_va":5372625024,"va":5371112519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc860
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1525223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8e7451700","size":5,"soft_fail":false,"target_va":5372627040,"va":5371101812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1643775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8ff141900","size":5,"soft_fail":false,"target_va":5372749696,"va":5371105916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1647795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8b3241900","size":5,"soft_fail":false,"target_va":5372749712,"va":5371101912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2048720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8d0421f00","size":5,"soft_fail":false,"target_va":5373149840,"va":5371101115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140455590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2133707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8cb8e2000","size":5,"soft_fail":false,"target_va":5373253008,"va":5371119296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4261426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e832064100","size":5,"soft_fail":false,"target_va":5375372576,"va":5371111145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31804554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e88a4ce501","size":5,"soft_fail":false,"target_va":5402900112,"va":5371095553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32497327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8afdeef01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371096028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32492144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e870caef01","size":5,"soft_fail":false,"target_va":5403593376,"va":5371101227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31855318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8d612e601","size":5,"soft_fail":false,"target_va":5402978144,"va":5371122821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t205118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e83e210300","size":5,"soft_fail":false,"target_va":5371305056,"va":5371099933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::Clipboard::get_text
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1724241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8514f1a00","size":5,"soft_fail":false,"target_va":5372833280,"va":5371109034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5845947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8bb335900","size":5,"soft_fail":false,"target_va":5376962688,"va":5371116736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_input::keyboard::impl$33::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8dfb8ffff","size":5,"soft_fail":false,"target_va":5371092176,"va":5371110380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31855705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e85914e601","size":5,"soft_fail":false,"target_va":5402978688,"va":5371122978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=35
//     evidence={"assembly":"\tcallq\t31857220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8441ae601","size":5,"soft_fail":false,"target_va":5402978880,"va":5371121655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31857523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8731be601","size":5,"soft_fail":false,"target_va":5402979152,"va":5371121624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31856379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8fb16e601","size":5,"soft_fail":false,"target_va":5402978944,"va":5371122560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-26289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e84f99ffff","size":5,"soft_fail":false,"target_va":5371090016,"va":5371116300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<winit::keyboard::Key<smol_str::SmolStr> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-40179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e80d63ffff","size":5,"soft_fail":false,"target_va":5371089952,"va":5371130126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31857232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8501ae601","size":5,"soft_fail":false,"target_va":5402980128,"va":5371122891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31857517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e86d1be601","size":5,"soft_fail":false,"target_va":5402980656,"va":5371123134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::finish_wait
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3488671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e89f3b3500","size":5,"soft_fail":false,"target_va":5374585584,"va":5371096908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::is_finished
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3488396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e88c3a3500","size":5,"soft_fail":false,"target_va":5374585568,"va":5371097167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::is_waiting
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3489260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8ec3d3500","size":5,"soft_fail":false,"target_va":5374585552,"va":5371096287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4304217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e859ad4100","size":5,"soft_fail":false,"target_va":5375412192,"va":5371107970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::into_disable_hold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4148087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8774b3f00","size":5,"soft_fail":false,"target_va":5375248672,"va":5371100580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::begin_frame
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5367417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e879e65100","size":5,"soft_fail":false,"target_va":5376483696,"va":5371116274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t6257245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e85d7a5f00","size":5,"soft_fail":false,"target_va":5377353776,"va":5371096526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] once_cell::imp::OnceCell$LT$T$GT$::initialize
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31807580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e85c58e501","size":5,"soft_fail":false,"target_va":5402921760,"va":5371114175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31856510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e87e17e601","size":5,"soft_fail":false,"target_va":5402970800,"va":5371114285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::GlobalProfiler::lock
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5244295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e887055000","size":5,"soft_fail":false,"target_va":5376358544,"va":5371114244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::GlobalProfiler::new_frame
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5244356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c4055000","size":5,"soft_fail":false,"target_va":5376358624,"va":5371114263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5242792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8a8ff4f00","size":5,"soft_fail":false,"target_va":5376356912,"va":5371114115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t5246825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8690f5000","size":5,"soft_fail":false,"target_va":5376360560,"va":5371113730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5242586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8dafe4f00","size":5,"soft_fail":false,"target_va":5376356144,"va":5371113553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5206778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8fa724f00","size":5,"soft_fail":false,"target_va":5376317936,"va":5371111153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5204408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8b8694f00","size":5,"soft_fail":false,"target_va":5376317648,"va":5371113235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::any_present_successful
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5160406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8d6bd4e00","size":5,"soft_fail":false,"target_va":5376278736,"va":5371118325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::extent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5166525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8bdd54e00","size":5,"soft_fail":false,"target_va":5376278016,"va":5371111486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::mark_stale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5171252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e834e84e00","size":5,"soft_fail":false,"target_va":5376277728,"va":5371106471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::on_focus
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5170381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8cde44e00","size":5,"soft_fail":false,"target_va":5376277104,"va":5371106718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::resize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5165859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e823d34e00","size":5,"soft_fail":false,"target_va":5376277376,"va":5371111512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::set_vsync
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5158284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e88cb54e00","size":5,"soft_fail":false,"target_va":5376277552,"va":5371119263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_pool::ResourcePool::clear
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5234607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8afdf4f00","size":5,"soft_fail":false,"target_va":5376347104,"va":5371112492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6253722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e89a6c5f00","size":5,"soft_fail":false,"target_va":5377366224,"va":5371112497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t31869327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e88f49e601","size":5,"soft_fail":false,"target_va":5402975328,"va":5371105996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1332796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c4a9ebff","size":5,"soft_fail":false,"target_va":5369769376,"va":5371102167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1315593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8f7ecebff","size":5,"soft_fail":false,"target_va":5369780768,"va":5371096356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t877509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8c5630d00","size":5,"soft_fail":false,"target_va":5371979584,"va":5371102070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t892790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8769f0d00","size":5,"soft_fail":false,"target_va":5371989136,"va":5371096341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1431428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e884d71500","size":5,"soft_fail":false,"target_va":5372533552,"va":5371102119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1473863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8477d1600","size":5,"soft_fail":false,"target_va":5372570240,"va":5371096372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31866267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e89b3de601","size":5,"soft_fail":false,"target_va":5402975504,"va":5371109232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31866554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8ba3ee601","size":5,"soft_fail":false,"target_va":5402975696,"va":5371109137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1511413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8f50f1700","size":5,"soft_fail":false,"target_va":5372625168,"va":5371113750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6282132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e894db5f00","size":5,"soft_fail":false,"target_va":5377393200,"va":5371111063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6329529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8b9946000","size":5,"soft_fail":false,"target_va":5377443760,"va":5371114226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::egui_backend::EguiBackend::end_frame
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2031124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8ec01e1ff","size":5,"soft_fail":false,"target_va":5369085664,"va":5371116783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::event_loop::EguiEventIo::take_egui_input
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t798668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8cc2f0c00","size":5,"soft_fail":false,"target_va":5371913488,"va":5371114815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fps_limiter::FpsLimiter::frame
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t721057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8a1000b00","size":5,"soft_fail":false,"target_va":5371839344,"va":5371118282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::game_loop::GameLoop::extract_available_monitors_info
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t36773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8a58f0000","size":5,"soft_fail":false,"target_va":5371132480,"va":5371095702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::game_loop::on_cursor_moved_absolute
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t33783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8f7830000","size":5,"soft_fail":false,"target_va":5371135520,"va":5371101732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::render_loop::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t743740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e83c590b00","size":5,"soft_fail":false,"target_va":5371861232,"va":5371117487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::setup::find_best_fullscreen_video_mode
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1437681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e80f10eaff","size":5,"soft_fail":false,"target_va":5369681776,"va":5371119452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::vram::calculate_max_resolution_scale
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t419968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e880680600","size":5,"soft_fail":false,"target_va":5371531440,"va":5371111467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::winit_bevy_glue::winit_to_bevy_keycode
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t44248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8d8ac0000","size":5,"soft_fail":false,"target_va":5371149312,"va":5371105059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::on_clipboard_paste
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2050330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e81a491f00","size":5,"soft_fail":false,"target_va":5373160656,"va":5371110321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::on_key_event
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2052619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e80b521f00","size":5,"soft_fail":false,"target_va":5373160416,"va":5371107792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::keyboard::Key::to_text
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2141495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e837ad2000","size":5,"soft_fail":false,"target_va":5373249328,"va":5371107828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1079383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e857781000","size":5,"soft_fail":false,"target_va":5372197792,"va":5371118404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2146135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e857bf2000","size":5,"soft_fail":false,"target_va":5373265648,"va":5371119508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::current_monitor
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2161329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8b1fa2000","size":5,"soft_fail":false,"target_va":5373265824,"va":5371104490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2147297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8e1c32000","size":5,"soft_fail":false,"target_va":5373253168,"va":5371105866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_fullscreen
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-29632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371095536,"raw_hex":"e8408cffff","size":5,"soft_fail":false,"target_va":5371090208,"va":5371119835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":52616,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":16044,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":127,"record_offset":52880,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":31804,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":43684,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":10430,"source_line":263,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":28072,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":41884,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::remove_resource
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":33744,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":10621,"source_line":1365,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":29936,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":7132,"source_line":1560,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":29824,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":127,"record_offset":29852,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":127,"record_offset":52852,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":29616,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":33460,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":35736,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":29908,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":127,"record_offset":29880,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":127,"record_offset":29740,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":127,"record_offset":29672,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":7942,"source_line":789,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::apply_setting_changes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":16072,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":10635,"source_line":708,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::process_app_event
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":53080,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":10649,"source_line":299,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::receive_app_events
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":47936,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":10639,"source_line":117,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::send_window_events
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":48172,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":10640,"source_line":134,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::simulate_and_render_frame
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":92356,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":10687,"source_line":492,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":125472,"parent_record_offset":15820,"ancestor_record_offset":15820,"inlinee":10696,"source_line":940,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":73784,"parent_record_offset":73760,"ancestor_record_offset":15820,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":73808,"parent_record_offset":73784,"ancestor_record_offset":15820,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":22012,"parent_record_offset":21984,"ancestor_record_offset":15820,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":73760,"parent_record_offset":73736,"ancestor_record_offset":15820,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":21984,"parent_record_offset":21864,"ancestor_record_offset":15820,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$3::write_clone_into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":73884,"parent_record_offset":73668,"ancestor_record_offset":15820,"inlinee":10526,"source_line":449,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":73668,"parent_record_offset":73640,"ancestor_record_offset":15820,"inlinee":7748,"source_line":1306,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":73704,"parent_record_offset":73668,"ancestor_record_offset":15820,"inlinee":7066,"source_line":443,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":73736,"parent_record_offset":73704,"ancestor_record_offset":15820,"inlinee":7067,"source_line":480,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":27616,"parent_record_offset":27592,"ancestor_record_offset":15820,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":21904,"parent_record_offset":21864,"ancestor_record_offset":15820,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":25028,"parent_record_offset":25000,"ancestor_record_offset":15820,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":21864,"parent_record_offset":21832,"ancestor_record_offset":15820,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":88312,"parent_record_offset":88276,"ancestor_record_offset":15820,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":80180,"parent_record_offset":80140,"ancestor_record_offset":15820,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":21304,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":88276,"parent_record_offset":88252,"ancestor_record_offset":15820,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":25000,"parent_record_offset":24956,"ancestor_record_offset":15820,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":52652,"parent_record_offset":52616,"ancestor_record_offset":15820,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":52708,"parent_record_offset":52616,"ancestor_record_offset":15820,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":24956,"parent_record_offset":24916,"ancestor_record_offset":15820,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":52736,"parent_record_offset":52616,"ancestor_record_offset":15820,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":50444,"parent_record_offset":48172,"ancestor_record_offset":15820,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":127,"record_offset":50412,"parent_record_offset":48172,"ancestor_record_offset":15820,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=25
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":22096,"parent_record_offset":22068,"ancestor_record_offset":15820,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=23
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":22068,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=44
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":17548,"parent_record_offset":17516,"ancestor_record_offset":15820,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":127,"record_offset":80336,"parent_record_offset":80004,"ancestor_record_offset":15820,"inlinee":10674,"source_line":145,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":24916,"parent_record_offset":24888,"ancestor_record_offset":15820,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send_default
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":24888,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":10430,"source_line":263,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=24
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19244,"parent_record_offset":19216,"ancestor_record_offset":15820,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=24
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19216,"parent_record_offset":19184,"ancestor_record_offset":15820,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=44
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19152,"parent_record_offset":19120,"ancestor_record_offset":15820,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":35280,"parent_record_offset":35176,"ancestor_record_offset":15820,"inlinee":10627,"source_line":294,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":21120,"parent_record_offset":20680,"ancestor_record_offset":15820,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=24
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19092,"parent_record_offset":18652,"ancestor_record_offset":15820,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=44
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19120,"parent_record_offset":19092,"ancestor_record_offset":15820,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":35176,"parent_record_offset":33744,"ancestor_record_offset":15820,"inlinee":10626,"source_line":210,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=38
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":18688,"parent_record_offset":18652,"ancestor_record_offset":15820,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":34804,"parent_record_offset":33744,"ancestor_record_offset":15820,"inlinee":10622,"source_line":297,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=24
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19184,"parent_record_offset":19120,"ancestor_record_offset":15820,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=44
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18744,"parent_record_offset":18716,"ancestor_record_offset":15820,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=38
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18716,"parent_record_offset":18688,"ancestor_record_offset":15820,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":34840,"parent_record_offset":34804,"ancestor_record_offset":15820,"inlinee":10623,"source_line":379,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=44
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18848,"parent_record_offset":18776,"ancestor_record_offset":15820,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=38
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18944,"parent_record_offset":18916,"ancestor_record_offset":15820,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":35044,"parent_record_offset":35020,"ancestor_record_offset":15820,"inlinee":10625,"source_line":381,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":119208,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":48280,"parent_record_offset":48172,"ancestor_record_offset":15820,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=20
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":29992,"parent_record_offset":29936,"ancestor_record_offset":15820,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::last_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":19324,"parent_record_offset":19292,"ancestor_record_offset":15820,"inlinee":9923,"source_line":2096,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=24
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":19356,"parent_record_offset":19292,"ancestor_record_offset":15820,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":120392,"parent_record_offset":120364,"ancestor_record_offset":15820,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":120784,"parent_record_offset":120756,"ancestor_record_offset":15820,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=23
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":24652,"parent_record_offset":24016,"ancestor_record_offset":15820,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":19540,"parent_record_offset":19504,"ancestor_record_offset":15820,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":20680,"parent_record_offset":19540,"ancestor_record_offset":15820,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=24
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":17516,"parent_record_offset":17484,"ancestor_record_offset":15820,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=24
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":18652,"parent_record_offset":17516,"ancestor_record_offset":15820,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::last_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":19292,"parent_record_offset":18652,"ancestor_record_offset":15820,"inlinee":9922,"source_line":268,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::impl$12::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/lib.rs
//     evidence={"module":127,"record_offset":80308,"parent_record_offset":80004,"ancestor_record_offset":15820,"inlinee":10672,"source_line":132,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$3::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":127,"record_offset":80004,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":10668,"source_line":90,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$47::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":127,"record_offset":80048,"parent_record_offset":80004,"ancestor_record_offset":15820,"inlinee":10670,"source_line":718,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":127,"record_offset":37748,"parent_record_offset":35780,"ancestor_record_offset":15820,"inlinee":10628,"source_line":428,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":18356,"parent_record_offset":18328,"ancestor_record_offset":15820,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":31736,"parent_record_offset":29936,"ancestor_record_offset":15820,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":127,"record_offset":97412,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":8003,"source_line":14,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":127,"record_offset":97468,"parent_record_offset":97440,"ancestor_record_offset":15820,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":127,"record_offset":97440,"parent_record_offset":97412,"ancestor_record_offset":15820,"inlinee":6955,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":108856,"parent_record_offset":108824,"ancestor_record_offset":15820,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":101876,"parent_record_offset":101848,"ancestor_record_offset":15820,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":101936,"parent_record_offset":101908,"ancestor_record_offset":15820,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":101908,"parent_record_offset":101848,"ancestor_record_offset":15820,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":108792,"parent_record_offset":108756,"ancestor_record_offset":15820,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":108824,"parent_record_offset":108792,"ancestor_record_offset":15820,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":109016,"parent_record_offset":108824,"ancestor_record_offset":15820,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":109136,"parent_record_offset":109112,"ancestor_record_offset":15820,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":127,"record_offset":80268,"parent_record_offset":80140,"ancestor_record_offset":15820,"inlinee":8568,"source_line":219,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":99424,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":5335,"source_line":896,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65808,"parent_record_offset":65784,"ancestor_record_offset":15820,"inlinee":8949,"source_line":1221,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":22656,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":18412,"parent_record_offset":18384,"ancestor_record_offset":15820,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":15820,"inlinee":10653,"source_line":1549,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65880,"parent_record_offset":65856,"ancestor_record_offset":15820,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":99484,"parent_record_offset":99456,"ancestor_record_offset":15820,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":18328,"parent_record_offset":18300,"ancestor_record_offset":15820,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":127,"record_offset":22196,"parent_record_offset":22132,"ancestor_record_offset":15820,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":127,"record_offset":22312,"parent_record_offset":22284,"ancestor_record_offset":15820,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":17764,"parent_record_offset":17736,"ancestor_record_offset":15820,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=88
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":17860,"parent_record_offset":17832,"ancestor_record_offset":15820,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":127,"record_offset":99336,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":127,"record_offset":99252,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":127,"record_offset":22688,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":15820,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":127,"record_offset":21448,"parent_record_offset":21420,"ancestor_record_offset":15820,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":127,"record_offset":21524,"parent_record_offset":21496,"ancestor_record_offset":15820,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":127,"record_offset":21496,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":127,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":15820,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":16896,"parent_record_offset":16864,"ancestor_record_offset":15820,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":35444,"parent_record_offset":35368,"ancestor_record_offset":15820,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":16864,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":103144,"parent_record_offset":103116,"ancestor_record_offset":15820,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":103172,"parent_record_offset":103144,"ancestor_record_offset":15820,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":21944,"parent_record_offset":21904,"ancestor_record_offset":15820,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":127,"record_offset":108248,"parent_record_offset":108148,"ancestor_record_offset":15820,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":102424,"parent_record_offset":102364,"ancestor_record_offset":15820,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":17980,"parent_record_offset":17952,"ancestor_record_offset":15820,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":127,"record_offset":77596,"parent_record_offset":77568,"ancestor_record_offset":15820,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":86472,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":15820,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18616,"parent_record_offset":17548,"ancestor_record_offset":15820,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18916,"parent_record_offset":18716,"ancestor_record_offset":15820,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":113796,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":108724,"parent_record_offset":108528,"ancestor_record_offset":15820,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":21560,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":122936,"parent_record_offset":122904,"ancestor_record_offset":15820,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":19464,"parent_record_offset":17484,"ancestor_record_offset":15820,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":82148,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":82172,"parent_record_offset":82148,"ancestor_record_offset":15820,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":25248,"parent_record_offset":25180,"ancestor_record_offset":15820,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":19036,"parent_record_offset":19008,"ancestor_record_offset":15820,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=163
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":17012,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":35124,"parent_record_offset":35100,"ancestor_record_offset":15820,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":73908,"parent_record_offset":73884,"ancestor_record_offset":15820,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":18232,"parent_record_offset":18204,"ancestor_record_offset":15820,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":37772,"parent_record_offset":37748,"ancestor_record_offset":15820,"inlinee":6972,"source_line":1335,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=89
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":18140,"parent_record_offset":18112,"ancestor_record_offset":15820,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":50492,"parent_record_offset":50468,"ancestor_record_offset":15820,"inlinee":10644,"source_line":1220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":50468,"parent_record_offset":50444,"ancestor_record_offset":15820,"inlinee":10643,"source_line":1242,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":50516,"parent_record_offset":50492,"ancestor_record_offset":15820,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":53008,"parent_record_offset":52984,"ancestor_record_offset":15820,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":88344,"parent_record_offset":88312,"ancestor_record_offset":15820,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":107676,"parent_record_offset":107648,"ancestor_record_offset":15820,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":107648,"parent_record_offset":107620,"ancestor_record_offset":15820,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":52952,"parent_record_offset":52908,"ancestor_record_offset":15820,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":16932,"parent_record_offset":16896,"ancestor_record_offset":15820,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":73836,"parent_record_offset":73808,"ancestor_record_offset":15820,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":35332,"parent_record_offset":35280,"ancestor_record_offset":15820,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":35368,"parent_record_offset":35332,"ancestor_record_offset":15820,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":16968,"parent_record_offset":16896,"ancestor_record_offset":15820,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":109528,"parent_record_offset":108484,"ancestor_record_offset":15820,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":16424,"parent_record_offset":16356,"ancestor_record_offset":15820,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":51232,"parent_record_offset":51176,"ancestor_record_offset":15820,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":86520,"parent_record_offset":86496,"ancestor_record_offset":15820,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":16816,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":127,"record_offset":82272,"parent_record_offset":82248,"ancestor_record_offset":15820,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":127,"record_offset":82296,"parent_record_offset":82272,"ancestor_record_offset":15820,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":18776,"parent_record_offset":18744,"ancestor_record_offset":15820,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":52764,"parent_record_offset":52616,"ancestor_record_offset":15820,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":18816,"parent_record_offset":18776,"ancestor_record_offset":15820,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":19008,"parent_record_offset":18972,"ancestor_record_offset":15820,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":35100,"parent_record_offset":35068,"ancestor_record_offset":15820,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":127,"record_offset":52788,"parent_record_offset":52764,"ancestor_record_offset":15820,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":52984,"parent_record_offset":52908,"ancestor_record_offset":15820,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":107620,"parent_record_offset":107592,"ancestor_record_offset":15820,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":107988,"parent_record_offset":107960,"ancestor_record_offset":15820,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":52908,"parent_record_offset":52880,"ancestor_record_offset":15820,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":107592,"parent_record_offset":107472,"ancestor_record_offset":15820,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":127,"record_offset":108148,"parent_record_offset":108116,"ancestor_record_offset":15820,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":127,"record_offset":107292,"parent_record_offset":107248,"ancestor_record_offset":15820,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":127,"record_offset":107440,"parent_record_offset":107408,"ancestor_record_offset":15820,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":127,"record_offset":107408,"parent_record_offset":107376,"ancestor_record_offset":15820,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":127,"record_offset":107348,"parent_record_offset":107320,"ancestor_record_offset":15820,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":127,"record_offset":107376,"parent_record_offset":107348,"ancestor_record_offset":15820,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":127,"record_offset":107320,"parent_record_offset":107292,"ancestor_record_offset":15820,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":127,"record_offset":107960,"parent_record_offset":107376,"ancestor_record_offset":15820,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108116,"parent_record_offset":108076,"ancestor_record_offset":15820,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108284,"parent_record_offset":108116,"ancestor_record_offset":15820,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108076,"parent_record_offset":108040,"ancestor_record_offset":15820,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":82224,"parent_record_offset":82200,"ancestor_record_offset":15820,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108040,"parent_record_offset":107248,"ancestor_record_offset":15820,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108312,"parent_record_offset":108284,"ancestor_record_offset":15820,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":127,"record_offset":107472,"parent_record_offset":107440,"ancestor_record_offset":15820,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":127,"record_offset":107752,"parent_record_offset":107472,"ancestor_record_offset":15820,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":127,"record_offset":107720,"parent_record_offset":107472,"ancestor_record_offset":15820,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":127,"record_offset":107528,"parent_record_offset":107472,"ancestor_record_offset":15820,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16728,"parent_record_offset":16696,"ancestor_record_offset":15820,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":17276,"parent_record_offset":17076,"ancestor_record_offset":15820,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":109776,"parent_record_offset":109748,"ancestor_record_offset":15820,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":15820,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16356,"parent_record_offset":16328,"ancestor_record_offset":15820,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":17376,"parent_record_offset":17344,"ancestor_record_offset":15820,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":80208,"parent_record_offset":80180,"ancestor_record_offset":15820,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16620,"parent_record_offset":16588,"ancestor_record_offset":15820,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":80232,"parent_record_offset":80208,"ancestor_record_offset":15820,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16392,"parent_record_offset":16356,"ancestor_record_offset":15820,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16648,"parent_record_offset":16620,"ancestor_record_offset":15820,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":17304,"parent_record_offset":17276,"ancestor_record_offset":15820,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":88372,"parent_record_offset":88312,"ancestor_record_offset":15820,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":17404,"parent_record_offset":17376,"ancestor_record_offset":15820,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":88404,"parent_record_offset":88276,"ancestor_record_offset":15820,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":18384,"parent_record_offset":18356,"ancestor_record_offset":15820,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":127,"record_offset":25284,"parent_record_offset":25248,"ancestor_record_offset":15820,"inlinee":10713,"source_line":29,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":127,"record_offset":25180,"parent_record_offset":25108,"ancestor_record_offset":15820,"inlinee":10709,"source_line":43,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$24::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":127,"record_offset":25108,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":10707,"source_line":75,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":127,"record_offset":25220,"parent_record_offset":25180,"ancestor_record_offset":15820,"inlinee":10711,"source_line":13,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":127,"record_offset":19504,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":127,"record_offset":17484,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":18300,"parent_record_offset":18276,"ancestor_record_offset":15820,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":17576,"parent_record_offset":17548,"ancestor_record_offset":15820,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":17604,"parent_record_offset":17576,"ancestor_record_offset":15820,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":18276,"parent_record_offset":18056,"ancestor_record_offset":15820,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":103084,"parent_record_offset":103048,"ancestor_record_offset":15820,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":102984,"parent_record_offset":102956,"ancestor_record_offset":15820,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":103048,"parent_record_offset":102984,"ancestor_record_offset":15820,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":103016,"parent_record_offset":102984,"ancestor_record_offset":15820,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":103116,"parent_record_offset":103084,"ancestor_record_offset":15820,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":102660,"parent_record_offset":102636,"ancestor_record_offset":15820,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18176,"parent_record_offset":18080,"ancestor_record_offset":15820,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18112,"parent_record_offset":18080,"ancestor_record_offset":15820,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18564,"parent_record_offset":17692,"ancestor_record_offset":15820,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":15820,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17664,"parent_record_offset":17636,"ancestor_record_offset":15820,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17636,"parent_record_offset":17604,"ancestor_record_offset":15820,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17692,"parent_record_offset":17664,"ancestor_record_offset":15820,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17924,"parent_record_offset":17896,"ancestor_record_offset":15820,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17952,"parent_record_offset":17924,"ancestor_record_offset":15820,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":18020,"parent_record_offset":17896,"ancestor_record_offset":15820,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17896,"parent_record_offset":17692,"ancestor_record_offset":15820,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18056,"parent_record_offset":17692,"ancestor_record_offset":15820,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":17736,"parent_record_offset":17692,"ancestor_record_offset":15820,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=88
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":17832,"parent_record_offset":17692,"ancestor_record_offset":15820,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":18468,"parent_record_offset":17692,"ancestor_record_offset":15820,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":110096,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":110184,"parent_record_offset":110160,"ancestor_record_offset":15820,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":127,"record_offset":22720,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":127,"record_offset":22560,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":18972,"parent_record_offset":18944,"ancestor_record_offset":15820,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":127,"record_offset":109748,"parent_record_offset":109720,"ancestor_record_offset":15820,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109632,"parent_record_offset":109600,"ancestor_record_offset":15820,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109720,"parent_record_offset":109688,"ancestor_record_offset":15820,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109660,"parent_record_offset":109632,"ancestor_record_offset":15820,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109688,"parent_record_offset":109660,"ancestor_record_offset":15820,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109600,"parent_record_offset":109572,"ancestor_record_offset":15820,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":127,"record_offset":110212,"parent_record_offset":110184,"ancestor_record_offset":15820,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":108396,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":108440,"parent_record_offset":108396,"ancestor_record_offset":15820,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":127,"record_offset":108576,"parent_record_offset":108528,"ancestor_record_offset":15820,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":108756,"parent_record_offset":108528,"ancestor_record_offset":15820,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":109056,"parent_record_offset":108756,"ancestor_record_offset":15820,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::now_ns
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":109572,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":6957,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":107248,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":127,"record_offset":25408,"parent_record_offset":25108,"ancestor_record_offset":15820,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":127,"record_offset":101536,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":5555,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":127,"record_offset":80116,"parent_record_offset":80048,"ancestor_record_offset":15820,"inlinee":10614,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$37::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":127,"record_offset":80140,"parent_record_offset":80116,"ancestor_record_offset":15820,"inlinee":10616,"source_line":410,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":127,"record_offset":102056,"parent_record_offset":101576,"ancestor_record_offset":15820,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":127,"record_offset":101576,"parent_record_offset":101536,"ancestor_record_offset":15820,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":127,"record_offset":101632,"parent_record_offset":101604,"ancestor_record_offset":15820,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":127,"record_offset":101720,"parent_record_offset":101688,"ancestor_record_offset":15820,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":127,"record_offset":101848,"parent_record_offset":101688,"ancestor_record_offset":15820,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":127,"record_offset":101604,"parent_record_offset":101576,"ancestor_record_offset":15820,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":127,"record_offset":16588,"parent_record_offset":16560,"ancestor_record_offset":15820,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":127,"record_offset":16560,"parent_record_offset":16532,"ancestor_record_offset":15820,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":127,"record_offset":29696,"parent_record_offset":29672,"ancestor_record_offset":15820,"inlinee":7892,"source_line":287,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":127,"record_offset":51176,"parent_record_offset":51136,"ancestor_record_offset":15820,"inlinee":10646,"source_line":125,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$4::send::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":127,"record_offset":51272,"parent_record_offset":51232,"ancestor_record_offset":15820,"inlinee":10648,"source_line":131,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":127,"record_offset":48100,"parent_record_offset":47936,"ancestor_record_offset":15820,"inlinee":7942,"source_line":789,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":127,"record_offset":49788,"parent_record_offset":48172,"ancestor_record_offset":15820,"inlinee":5570,"source_line":612,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":16292,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":16464,"parent_record_offset":16292,"ancestor_record_offset":15820,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":17040,"parent_record_offset":17012,"ancestor_record_offset":15820,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":17076,"parent_record_offset":17040,"ancestor_record_offset":15820,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":16696,"parent_record_offset":16496,"ancestor_record_offset":15820,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":16496,"parent_record_offset":16464,"ancestor_record_offset":15820,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":127,"record_offset":16328,"parent_record_offset":16292,"ancestor_record_offset":15820,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":127,"record_offset":17344,"parent_record_offset":17040,"ancestor_record_offset":15820,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":127,"record_offset":101748,"parent_record_offset":101720,"ancestor_record_offset":15820,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":127,"record_offset":101776,"parent_record_offset":101748,"ancestor_record_offset":15820,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":127,"record_offset":101688,"parent_record_offset":101660,"ancestor_record_offset":15820,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":127,"record_offset":101660,"parent_record_offset":101632,"ancestor_record_offset":15820,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":127,"record_offset":16532,"parent_record_offset":16496,"ancestor_record_offset":15820,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::PendingVideoSettingChanges::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":127,"record_offset":21272,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":10701,"source_line":603,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::PendingVideoSettingChanges::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":127,"record_offset":17452,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":10700,"source_line":585,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::impl$25::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":127,"record_offset":87500,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":10686,"source_line":739,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::egui_backend::EguiBackend::take_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":127,"record_offset":113856,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":7355,"source_line":45,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::event_loop::impl$5::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":127,"record_offset":65832,"parent_record_offset":65808,"ancestor_record_offset":15820,"inlinee":10652,"source_line":72,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":65784,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":10650,"source_line":944,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::next_request_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":49760,"parent_record_offset":48172,"ancestor_record_offset":15820,"inlinee":10642,"source_line":935,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::on_confirmation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":71276,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":10662,"source_line":940,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::apply_setting_changes::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":21592,"parent_record_offset":21560,"ancestor_record_offset":15820,"inlinee":10705,"source_line":735,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::main_loop::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":29640,"parent_record_offset":29616,"ancestor_record_offset":15820,"inlinee":10637,"source_line":190,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::main_loop::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":33428,"parent_record_offset":33404,"ancestor_record_offset":15820,"inlinee":10638,"source_line":210,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":86496,"parent_record_offset":86472,"ancestor_record_offset":15820,"inlinee":10680,"source_line":382,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":113824,"parent_record_offset":113796,"ancestor_record_offset":15820,"inlinee":10689,"source_line":623,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::log_time_to_first_frame
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":127,"record_offset":122904,"parent_record_offset":92356,"ancestor_record_offset":15820,"inlinee":10695,"source_line":44,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::impl$1::from_winit
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":127,"record_offset":22132,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":6079,"source_line":206,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":127,"record_offset":22256,"parent_record_offset":22224,"ancestor_record_offset":15820,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":127,"record_offset":22224,"parent_record_offset":22196,"ancestor_record_offset":15820,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::event::impl$80::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event.rs
//     evidence={"module":127,"record_offset":86444,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":10678,"source_line":1052,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::ModifiersState::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":127,"record_offset":69372,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":10659,"source_line":698,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":127,"record_offset":69404,"parent_record_offset":69372,"ancestor_record_offset":15820,"inlinee":10661,"source_line":698,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::impl$13::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/keyboard.rs
//     evidence={"module":127,"record_offset":86416,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":10676,"source_line":1504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":127,"record_offset":22164,"parent_record_offset":22132,"ancestor_record_offset":15820,"inlinee":6081,"source_line":51,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$1::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":127,"record_offset":74460,"parent_record_offset":74436,"ancestor_record_offset":15820,"inlinee":10684,"source_line":46,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":127,"record_offset":73608,"parent_record_offset":73568,"ancestor_record_offset":15820,"inlinee":10562,"source_line":29,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":127,"record_offset":73640,"parent_record_offset":73608,"ancestor_record_offset":15820,"inlinee":10518,"source_line":28,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":21696,"parent_record_offset":21672,"ancestor_record_offset":15820,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":73512,"parent_record_offset":73468,"ancestor_record_offset":15820,"inlinee":10666,"source_line":676,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":125500,"parent_record_offset":125472,"ancestor_record_offset":15820,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":21364,"parent_record_offset":21336,"ancestor_record_offset":15820,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":125552,"parent_record_offset":125528,"ancestor_record_offset":15820,"inlinee":10528,"source_line":143,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::available_monitors
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21336,"parent_record_offset":16072,"ancestor_record_offset":15820,"inlinee":10702,"source_line":1491,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":15820,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":73380,"parent_record_offset":53080,"ancestor_record_offset":15820,"inlinee":10663,"source_line":1099,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":73468,"parent_record_offset":73424,"ancestor_record_offset":15820,"inlinee":10665,"source_line":1101,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":74412,"parent_record_offset":74384,"ancestor_record_offset":15820,"inlinee":10682,"source_line":1101,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":125528,"parent_record_offset":125500,"ancestor_record_offset":15820,"inlinee":10698,"source_line":942,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::available_monitors::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21392,"parent_record_offset":21364,"ancestor_record_offset":15820,"inlinee":10704,"source_line":1492,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21672,"parent_record_offset":21644,"ancestor_record_offset":15820,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::main_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x298D
// 首条位置: {"module":127,"inlinee":10637}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::main_loop::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x298E
// 首条位置: {"module":127,"inlinee":10638}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::process_app_event
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 299
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2999
// 首条位置: {"module":127,"inlinee":10649}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":73352,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":73072,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":73128,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::send_default
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":75420,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10430,"source_line":263,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":62448,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":53608,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::keyboard::impl$3::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":127,"record_offset":80004,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10668,"source_line":90,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":127,"record_offset":85948,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":86472,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":77540,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":77568,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":90692,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":82148,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":77684,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":87456,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":86952,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::impl$25::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":127,"record_offset":87500,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10686,"source_line":739,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":65784,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10650,"source_line":944,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::on_confirmation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":71276,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10662,"source_line":940,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::event::impl$80::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event.rs
//     evidence={"module":127,"record_offset":86444,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10678,"source_line":1052,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::keyboard::ModifiersState::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":127,"record_offset":69372,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10659,"source_line":698,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::keyboard::impl$13::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/keyboard.rs
//     evidence={"module":127,"record_offset":86416,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10676,"source_line":1504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":65924,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":73380,"parent_record_offset":53080,"ancestor_record_offset":53080,"inlinee":10663,"source_line":1099,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":73784,"parent_record_offset":73760,"ancestor_record_offset":53080,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":73808,"parent_record_offset":73784,"ancestor_record_offset":53080,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":75032,"parent_record_offset":75004,"ancestor_record_offset":53080,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":73760,"parent_record_offset":73736,"ancestor_record_offset":53080,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":75004,"parent_record_offset":74944,"ancestor_record_offset":53080,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$3::write_clone_into_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":73884,"parent_record_offset":73668,"ancestor_record_offset":53080,"inlinee":10526,"source_line":449,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":73668,"parent_record_offset":73640,"ancestor_record_offset":53080,"inlinee":7748,"source_line":1306,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":73704,"parent_record_offset":73668,"ancestor_record_offset":53080,"inlinee":7066,"source_line":443,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":73736,"parent_record_offset":73704,"ancestor_record_offset":53080,"inlinee":7067,"source_line":480,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":86848,"parent_record_offset":86824,"ancestor_record_offset":53080,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":74972,"parent_record_offset":74944,"ancestor_record_offset":53080,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":73228,"parent_record_offset":73196,"ancestor_record_offset":53080,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":74944,"parent_record_offset":74920,"ancestor_record_offset":53080,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":88312,"parent_record_offset":88276,"ancestor_record_offset":53080,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":80180,"parent_record_offset":80140,"ancestor_record_offset":53080,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":88276,"parent_record_offset":88252,"ancestor_record_offset":53080,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":73196,"parent_record_offset":73160,"ancestor_record_offset":53080,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":73160,"parent_record_offset":73128,"ancestor_record_offset":53080,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":73096,"parent_record_offset":73072,"ancestor_record_offset":53080,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":53676,"parent_record_offset":53640,"ancestor_record_offset":53080,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":127,"record_offset":80336,"parent_record_offset":80004,"ancestor_record_offset":53080,"inlinee":10674,"source_line":145,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":75444,"parent_record_offset":75420,"ancestor_record_offset":53080,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":55208,"parent_record_offset":55184,"ancestor_record_offset":53080,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":55184,"parent_record_offset":55160,"ancestor_record_offset":53080,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":55132,"parent_record_offset":55100,"ancestor_record_offset":53080,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":63912,"parent_record_offset":63524,"ancestor_record_offset":53080,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":55076,"parent_record_offset":54684,"ancestor_record_offset":53080,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":55100,"parent_record_offset":55076,"ancestor_record_offset":53080,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":54716,"parent_record_offset":54684,"ancestor_record_offset":53080,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":55160,"parent_record_offset":55100,"ancestor_record_offset":53080,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":54764,"parent_record_offset":54740,"ancestor_record_offset":53080,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":54740,"parent_record_offset":54716,"ancestor_record_offset":53080,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":54856,"parent_record_offset":54792,"ancestor_record_offset":53080,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":54944,"parent_record_offset":54920,"ancestor_record_offset":53080,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":55276,"parent_record_offset":55252,"ancestor_record_offset":53080,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":55252,"parent_record_offset":54684,"ancestor_record_offset":53080,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":62480,"parent_record_offset":62448,"ancestor_record_offset":53080,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":63524,"parent_record_offset":62480,"ancestor_record_offset":53080,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":53640,"parent_record_offset":53608,"ancestor_record_offset":53080,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":54684,"parent_record_offset":53640,"ancestor_record_offset":53080,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::impl$12::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/lib.rs
//     evidence={"module":127,"record_offset":80308,"parent_record_offset":80004,"ancestor_record_offset":53080,"inlinee":10672,"source_line":132,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$47::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/keyboard.rs
//     evidence={"module":127,"record_offset":80048,"parent_record_offset":80004,"ancestor_record_offset":53080,"inlinee":10670,"source_line":718,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":54416,"parent_record_offset":54392,"ancestor_record_offset":53080,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":127,"record_offset":80268,"parent_record_offset":80140,"ancestor_record_offset":53080,"inlinee":8568,"source_line":219,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65808,"parent_record_offset":65784,"ancestor_record_offset":53080,"inlinee":8949,"source_line":1221,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":54464,"parent_record_offset":54440,"ancestor_record_offset":53080,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":53080,"inlinee":10653,"source_line":1549,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65880,"parent_record_offset":65856,"ancestor_record_offset":53080,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":54392,"parent_record_offset":54368,"ancestor_record_offset":53080,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":127,"record_offset":74436,"parent_record_offset":74412,"ancestor_record_offset":53080,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":127,"record_offset":86048,"parent_record_offset":86024,"ancestor_record_offset":53080,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":53880,"parent_record_offset":53856,"ancestor_record_offset":53080,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":53964,"parent_record_offset":53940,"ancestor_record_offset":53080,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":53904,"parent_record_offset":53880,"ancestor_record_offset":53080,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":54076,"parent_record_offset":54052,"ancestor_record_offset":53080,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":127,"record_offset":77596,"parent_record_offset":77568,"ancestor_record_offset":53080,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":54880,"parent_record_offset":54856,"ancestor_record_offset":53080,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":54652,"parent_record_offset":53676,"ancestor_record_offset":53080,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":54920,"parent_record_offset":54740,"ancestor_record_offset":53080,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":82172,"parent_record_offset":82148,"ancestor_record_offset":53080,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":73568,"parent_record_offset":73512,"ancestor_record_offset":53080,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":55024,"parent_record_offset":55000,"ancestor_record_offset":53080,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":73984,"parent_record_offset":73512,"ancestor_record_offset":53080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":73908,"parent_record_offset":73884,"ancestor_record_offset":53080,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":54304,"parent_record_offset":54280,"ancestor_record_offset":53080,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":54224,"parent_record_offset":54200,"ancestor_record_offset":53080,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":88344,"parent_record_offset":88312,"ancestor_record_offset":53080,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":73836,"parent_record_offset":73808,"ancestor_record_offset":53080,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":73260,"parent_record_offset":73160,"ancestor_record_offset":53080,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":87104,"parent_record_offset":87020,"ancestor_record_offset":53080,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":86520,"parent_record_offset":86496,"ancestor_record_offset":53080,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":127,"record_offset":82272,"parent_record_offset":82248,"ancestor_record_offset":53080,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":127,"record_offset":82296,"parent_record_offset":82272,"ancestor_record_offset":53080,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":54792,"parent_record_offset":54764,"ancestor_record_offset":53080,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":54828,"parent_record_offset":54792,"ancestor_record_offset":53080,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":55000,"parent_record_offset":54968,"ancestor_record_offset":53080,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":82224,"parent_record_offset":82200,"ancestor_record_offset":53080,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":87376,"parent_record_offset":87348,"ancestor_record_offset":53080,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":74224,"parent_record_offset":74044,"ancestor_record_offset":53080,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":55300,"parent_record_offset":55276,"ancestor_record_offset":53080,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":87020,"parent_record_offset":86988,"ancestor_record_offset":53080,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":74312,"parent_record_offset":74284,"ancestor_record_offset":53080,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":80208,"parent_record_offset":80180,"ancestor_record_offset":53080,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":74156,"parent_record_offset":74128,"ancestor_record_offset":53080,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":80232,"parent_record_offset":80208,"ancestor_record_offset":53080,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":87072,"parent_record_offset":87020,"ancestor_record_offset":53080,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":55324,"parent_record_offset":55300,"ancestor_record_offset":53080,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":74248,"parent_record_offset":74224,"ancestor_record_offset":53080,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":88372,"parent_record_offset":88312,"ancestor_record_offset":53080,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":74336,"parent_record_offset":74312,"ancestor_record_offset":53080,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":88404,"parent_record_offset":88276,"ancestor_record_offset":53080,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":54440,"parent_record_offset":54416,"ancestor_record_offset":53080,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":54368,"parent_record_offset":54344,"ancestor_record_offset":53080,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":53708,"parent_record_offset":53676,"ancestor_record_offset":53080,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":53732,"parent_record_offset":53708,"ancestor_record_offset":53080,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":54344,"parent_record_offset":54144,"ancestor_record_offset":53080,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":54256,"parent_record_offset":54172,"ancestor_record_offset":53080,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":54200,"parent_record_offset":54172,"ancestor_record_offset":53080,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":54600,"parent_record_offset":53808,"ancestor_record_offset":53080,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":54172,"parent_record_offset":54144,"ancestor_record_offset":53080,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":53784,"parent_record_offset":53760,"ancestor_record_offset":53080,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":53760,"parent_record_offset":53732,"ancestor_record_offset":53080,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":53808,"parent_record_offset":53784,"ancestor_record_offset":53080,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":54024,"parent_record_offset":53996,"ancestor_record_offset":53080,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":54052,"parent_record_offset":54024,"ancestor_record_offset":53080,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":54112,"parent_record_offset":53996,"ancestor_record_offset":53080,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":53996,"parent_record_offset":53808,"ancestor_record_offset":53080,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":54144,"parent_record_offset":53808,"ancestor_record_offset":53080,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":53856,"parent_record_offset":53808,"ancestor_record_offset":53080,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":53940,"parent_record_offset":53808,"ancestor_record_offset":53080,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":54516,"parent_record_offset":53808,"ancestor_record_offset":53080,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":54968,"parent_record_offset":54944,"ancestor_record_offset":53080,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":127,"record_offset":80116,"parent_record_offset":80048,"ancestor_record_offset":53080,"inlinee":10614,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$37::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":127,"record_offset":80140,"parent_record_offset":80116,"ancestor_record_offset":53080,"inlinee":10616,"source_line":410,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":127,"record_offset":74128,"parent_record_offset":74104,"ancestor_record_offset":53080,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":127,"record_offset":74104,"parent_record_offset":74080,"ancestor_record_offset":53080,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":87144,"parent_record_offset":86952,"ancestor_record_offset":53080,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":74012,"parent_record_offset":73984,"ancestor_record_offset":53080,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":74044,"parent_record_offset":74012,"ancestor_record_offset":53080,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":87348,"parent_record_offset":87172,"ancestor_record_offset":53080,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":87172,"parent_record_offset":87144,"ancestor_record_offset":53080,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":127,"record_offset":86988,"parent_record_offset":86952,"ancestor_record_offset":53080,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":127,"record_offset":74284,"parent_record_offset":74012,"ancestor_record_offset":53080,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":127,"record_offset":74080,"parent_record_offset":74044,"ancestor_record_offset":53080,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::event_loop::impl$5::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":127,"record_offset":65832,"parent_record_offset":65808,"ancestor_record_offset":53080,"inlinee":10652,"source_line":72,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":86496,"parent_record_offset":86472,"ancestor_record_offset":53080,"inlinee":10680,"source_line":382,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":127,"record_offset":86000,"parent_record_offset":85972,"ancestor_record_offset":53080,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":127,"record_offset":85972,"parent_record_offset":85948,"ancestor_record_offset":53080,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":127,"record_offset":69404,"parent_record_offset":69372,"ancestor_record_offset":53080,"inlinee":10661,"source_line":698,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$1::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":127,"record_offset":74460,"parent_record_offset":74436,"ancestor_record_offset":53080,"inlinee":10684,"source_line":46,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/mod.rs
//     evidence={"module":127,"record_offset":73608,"parent_record_offset":73568,"ancestor_record_offset":53080,"inlinee":10562,"source_line":29,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":127,"record_offset":73640,"parent_record_offset":73608,"ancestor_record_offset":53080,"inlinee":10518,"source_line":28,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":65996,"parent_record_offset":65972,"ancestor_record_offset":53080,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":73512,"parent_record_offset":73468,"ancestor_record_offset":53080,"inlinee":10666,"source_line":676,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":65948,"parent_record_offset":65924,"ancestor_record_offset":53080,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":73468,"parent_record_offset":73424,"ancestor_record_offset":53080,"inlinee":10665,"source_line":1101,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":74412,"parent_record_offset":74384,"ancestor_record_offset":53080,"inlinee":10682,"source_line":1101,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":65972,"parent_record_offset":65948,"ancestor_record_offset":53080,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::process_app_event::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x29B8
// 首条位置: {"module":127,"inlinee":10680}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":86520,"parent_record_offset":86496,"ancestor_record_offset":86496,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":86668,"parent_record_offset":86644,"ancestor_record_offset":86496,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":86644,"parent_record_offset":86620,"ancestor_record_offset":86496,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":86548,"parent_record_offset":86520,"ancestor_record_offset":86496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::simulate_and_render_frame
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 492
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x29BF
// 首条位置: {"module":127,"inlinee":10687}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":99280,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::contains_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":119208,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":10690,"source_line":1397,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":92776,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":97508,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":127,"record_offset":97412,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":8003,"source_line":14,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":99424,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":5335,"source_line":896,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":127,"record_offset":99336,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":127,"record_offset":99252,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":119172,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":113796,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":124988,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":102708,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":110096,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":108396,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] puffin::now_ns
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":109572,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":6957,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":107248,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":127,"record_offset":101536,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":5555,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":127,"record_offset":118508,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":5570,"source_line":612,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::egui_backend::EguiBackend::take_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":127,"record_offset":113856,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":7355,"source_line":45,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::log_time_to_first_frame
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/main.rs
//     evidence={"module":127,"record_offset":122904,"parent_record_offset":92356,"ancestor_record_offset":92356,"inlinee":10695,"source_line":44,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":103316,"parent_record_offset":103284,"ancestor_record_offset":92356,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":103284,"parent_record_offset":103048,"ancestor_record_offset":92356,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":119040,"parent_record_offset":119004,"ancestor_record_offset":92356,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":118800,"parent_record_offset":118776,"ancestor_record_offset":92356,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":99304,"parent_record_offset":99280,"ancestor_record_offset":92356,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":92828,"parent_record_offset":92800,"ancestor_record_offset":92356,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":99084,"parent_record_offset":99060,"ancestor_record_offset":92356,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":99060,"parent_record_offset":99036,"ancestor_record_offset":92356,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":94272,"parent_record_offset":94240,"ancestor_record_offset":92356,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":94216,"parent_record_offset":93828,"ancestor_record_offset":92356,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":98952,"parent_record_offset":98560,"ancestor_record_offset":92356,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":94240,"parent_record_offset":94216,"ancestor_record_offset":92356,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":93856,"parent_record_offset":93828,"ancestor_record_offset":92356,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":99036,"parent_record_offset":98976,"ancestor_record_offset":92356,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":93904,"parent_record_offset":93880,"ancestor_record_offset":92356,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":93880,"parent_record_offset":93856,"ancestor_record_offset":92356,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":93996,"parent_record_offset":93932,"ancestor_record_offset":92356,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":94084,"parent_record_offset":94060,"ancestor_record_offset":92356,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":99152,"parent_record_offset":99128,"ancestor_record_offset":92356,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":120392,"parent_record_offset":120364,"ancestor_record_offset":92356,"inlinee":10692,"source_line":1400,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$1
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":120784,"parent_record_offset":120756,"ancestor_record_offset":92356,"inlinee":10693,"source_line":1401,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":99128,"parent_record_offset":98560,"ancestor_record_offset":92356,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":92800,"parent_record_offset":92776,"ancestor_record_offset":92356,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":93828,"parent_record_offset":92800,"ancestor_record_offset":92356,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":97532,"parent_record_offset":97508,"ancestor_record_offset":92356,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":98560,"parent_record_offset":97532,"ancestor_record_offset":92356,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":93560,"parent_record_offset":93536,"ancestor_record_offset":92356,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":127,"record_offset":97468,"parent_record_offset":97440,"ancestor_record_offset":92356,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":127,"record_offset":97440,"parent_record_offset":97412,"ancestor_record_offset":92356,"inlinee":6955,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":108856,"parent_record_offset":108824,"ancestor_record_offset":92356,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":101876,"parent_record_offset":101848,"ancestor_record_offset":92356,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":101936,"parent_record_offset":101908,"ancestor_record_offset":92356,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":101908,"parent_record_offset":101848,"ancestor_record_offset":92356,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":108792,"parent_record_offset":108756,"ancestor_record_offset":92356,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":108824,"parent_record_offset":108792,"ancestor_record_offset":92356,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":109016,"parent_record_offset":108824,"ancestor_record_offset":92356,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":109136,"parent_record_offset":109112,"ancestor_record_offset":92356,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":123160,"parent_record_offset":122904,"ancestor_record_offset":92356,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":93608,"parent_record_offset":93584,"ancestor_record_offset":92356,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":99484,"parent_record_offset":99456,"ancestor_record_offset":92356,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":93536,"parent_record_offset":93512,"ancestor_record_offset":92356,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":93024,"parent_record_offset":93000,"ancestor_record_offset":92356,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":93108,"parent_record_offset":93084,"ancestor_record_offset":92356,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":127,"record_offset":99388,"parent_record_offset":99336,"ancestor_record_offset":92356,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":93048,"parent_record_offset":93024,"ancestor_record_offset":92356,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":101964,"parent_record_offset":101936,"ancestor_record_offset":92356,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":103144,"parent_record_offset":103116,"ancestor_record_offset":92356,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":103172,"parent_record_offset":103144,"ancestor_record_offset":92356,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":127,"record_offset":108248,"parent_record_offset":108148,"ancestor_record_offset":92356,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":102424,"parent_record_offset":102364,"ancestor_record_offset":92356,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":93220,"parent_record_offset":93196,"ancestor_record_offset":92356,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":120364,"parent_record_offset":119208,"ancestor_record_offset":92356,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":94020,"parent_record_offset":93996,"ancestor_record_offset":92356,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":93796,"parent_record_offset":92828,"ancestor_record_offset":92356,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":94060,"parent_record_offset":93880,"ancestor_record_offset":92356,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":108724,"parent_record_offset":108528,"ancestor_record_offset":92356,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":122936,"parent_record_offset":122904,"ancestor_record_offset":92356,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":94164,"parent_record_offset":94140,"ancestor_record_offset":92356,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":102588,"parent_record_offset":101536,"ancestor_record_offset":92356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":93448,"parent_record_offset":93424,"ancestor_record_offset":92356,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":93368,"parent_record_offset":93344,"ancestor_record_offset":92356,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":107676,"parent_record_offset":107648,"ancestor_record_offset":92356,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":107648,"parent_record_offset":107620,"ancestor_record_offset":92356,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":107880,"parent_record_offset":107848,"ancestor_record_offset":92356,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":122992,"parent_record_offset":122964,"ancestor_record_offset":92356,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":101992,"parent_record_offset":101964,"ancestor_record_offset":92356,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":109528,"parent_record_offset":108484,"ancestor_record_offset":92356,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":110308,"parent_record_offset":110212,"ancestor_record_offset":92356,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":93932,"parent_record_offset":93904,"ancestor_record_offset":92356,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":93968,"parent_record_offset":93932,"ancestor_record_offset":92356,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":94140,"parent_record_offset":94108,"ancestor_record_offset":92356,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":107620,"parent_record_offset":107592,"ancestor_record_offset":92356,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":107988,"parent_record_offset":107960,"ancestor_record_offset":92356,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":107592,"parent_record_offset":107472,"ancestor_record_offset":92356,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":127,"record_offset":108148,"parent_record_offset":108116,"ancestor_record_offset":92356,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":127,"record_offset":107292,"parent_record_offset":107248,"ancestor_record_offset":92356,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":127,"record_offset":107440,"parent_record_offset":107408,"ancestor_record_offset":92356,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":127,"record_offset":107408,"parent_record_offset":107376,"ancestor_record_offset":92356,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":127,"record_offset":107348,"parent_record_offset":107320,"ancestor_record_offset":92356,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":127,"record_offset":107376,"parent_record_offset":107348,"ancestor_record_offset":92356,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":127,"record_offset":107320,"parent_record_offset":107292,"ancestor_record_offset":92356,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":127,"record_offset":107960,"parent_record_offset":107376,"ancestor_record_offset":92356,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108116,"parent_record_offset":108076,"ancestor_record_offset":92356,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108284,"parent_record_offset":108116,"ancestor_record_offset":92356,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108076,"parent_record_offset":108040,"ancestor_record_offset":92356,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108040,"parent_record_offset":107248,"ancestor_record_offset":92356,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":127,"record_offset":108312,"parent_record_offset":108284,"ancestor_record_offset":92356,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":127,"record_offset":107472,"parent_record_offset":107440,"ancestor_record_offset":92356,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":127,"record_offset":107752,"parent_record_offset":107472,"ancestor_record_offset":92356,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":127,"record_offset":107720,"parent_record_offset":107472,"ancestor_record_offset":92356,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":127,"record_offset":107528,"parent_record_offset":107472,"ancestor_record_offset":92356,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":109776,"parent_record_offset":109748,"ancestor_record_offset":92356,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":99176,"parent_record_offset":99152,"ancestor_record_offset":92356,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":110244,"parent_record_offset":110212,"ancestor_record_offset":92356,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":123092,"parent_record_offset":123064,"ancestor_record_offset":92356,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":110272,"parent_record_offset":110244,"ancestor_record_offset":92356,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":99200,"parent_record_offset":99176,"ancestor_record_offset":92356,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":93584,"parent_record_offset":93560,"ancestor_record_offset":92356,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":93512,"parent_record_offset":93488,"ancestor_record_offset":92356,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":92852,"parent_record_offset":92828,"ancestor_record_offset":92356,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":92876,"parent_record_offset":92852,"ancestor_record_offset":92356,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":93488,"parent_record_offset":93288,"ancestor_record_offset":92356,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":103084,"parent_record_offset":103048,"ancestor_record_offset":92356,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":102984,"parent_record_offset":102956,"ancestor_record_offset":92356,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":103048,"parent_record_offset":102984,"ancestor_record_offset":92356,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":103016,"parent_record_offset":102984,"ancestor_record_offset":92356,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":103116,"parent_record_offset":103084,"ancestor_record_offset":92356,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":127,"record_offset":102660,"parent_record_offset":102636,"ancestor_record_offset":92356,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":93400,"parent_record_offset":93316,"ancestor_record_offset":92356,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":93344,"parent_record_offset":93316,"ancestor_record_offset":92356,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":93744,"parent_record_offset":92952,"ancestor_record_offset":92356,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":93316,"parent_record_offset":93288,"ancestor_record_offset":92356,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":92928,"parent_record_offset":92904,"ancestor_record_offset":92356,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":92904,"parent_record_offset":92876,"ancestor_record_offset":92356,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":92952,"parent_record_offset":92928,"ancestor_record_offset":92356,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":93168,"parent_record_offset":93140,"ancestor_record_offset":92356,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":93196,"parent_record_offset":93168,"ancestor_record_offset":92356,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":93256,"parent_record_offset":93140,"ancestor_record_offset":92356,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":93140,"parent_record_offset":92952,"ancestor_record_offset":92356,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":93288,"parent_record_offset":92952,"ancestor_record_offset":92356,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":93000,"parent_record_offset":92952,"ancestor_record_offset":92356,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=24
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":93084,"parent_record_offset":92952,"ancestor_record_offset":92356,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":93660,"parent_record_offset":92952,"ancestor_record_offset":92356,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":110184,"parent_record_offset":110160,"ancestor_record_offset":92356,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":127,"record_offset":123224,"parent_record_offset":122904,"ancestor_record_offset":92356,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":127,"record_offset":123064,"parent_record_offset":122904,"ancestor_record_offset":92356,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":94108,"parent_record_offset":94084,"ancestor_record_offset":92356,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":127,"record_offset":109748,"parent_record_offset":109720,"ancestor_record_offset":92356,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109632,"parent_record_offset":109600,"ancestor_record_offset":92356,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109720,"parent_record_offset":109688,"ancestor_record_offset":92356,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109660,"parent_record_offset":109632,"ancestor_record_offset":92356,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109688,"parent_record_offset":109660,"ancestor_record_offset":92356,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":127,"record_offset":109600,"parent_record_offset":109572,"ancestor_record_offset":92356,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":127,"record_offset":110212,"parent_record_offset":110184,"ancestor_record_offset":92356,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":108440,"parent_record_offset":108396,"ancestor_record_offset":92356,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":127,"record_offset":108576,"parent_record_offset":108528,"ancestor_record_offset":92356,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":108756,"parent_record_offset":108528,"ancestor_record_offset":92356,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":127,"record_offset":109056,"parent_record_offset":108756,"ancestor_record_offset":92356,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":127,"record_offset":102056,"parent_record_offset":101576,"ancestor_record_offset":92356,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":127,"record_offset":101576,"parent_record_offset":101536,"ancestor_record_offset":92356,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":127,"record_offset":101632,"parent_record_offset":101604,"ancestor_record_offset":92356,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":127,"record_offset":101720,"parent_record_offset":101688,"ancestor_record_offset":92356,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":127,"record_offset":101848,"parent_record_offset":101688,"ancestor_record_offset":92356,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":127,"record_offset":101604,"parent_record_offset":101576,"ancestor_record_offset":92356,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":127,"record_offset":101748,"parent_record_offset":101720,"ancestor_record_offset":92356,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":127,"record_offset":101776,"parent_record_offset":101748,"ancestor_record_offset":92356,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":127,"record_offset":101688,"parent_record_offset":101660,"ancestor_record_offset":92356,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":127,"record_offset":101660,"parent_record_offset":101632,"ancestor_record_offset":92356,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":113824,"parent_record_offset":113796,"ancestor_record_offset":92356,"inlinee":10689,"source_line":623,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 623
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x29C1
// 首条位置: {"module":127,"inlinee":10689}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::apply_setting_changes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 708
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x298B
// 首条位置: {"module":127,"inlinee":10635}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":21304,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":22068,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":24888,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":10430,"source_line":263,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":22656,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":127,"record_offset":22688,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":127,"record_offset":21496,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":16864,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":21560,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":17012,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":16816,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::render_config::impl$24::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":127,"record_offset":25108,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":10707,"source_line":75,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":127,"record_offset":19504,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":127,"record_offset":17484,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":127,"record_offset":22720,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":127,"record_offset":22560,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":16292,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::PendingVideoSettingChanges::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":127,"record_offset":21272,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":10701,"source_line":603,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::PendingVideoSettingChanges::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":127,"record_offset":17452,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":10700,"source_line":585,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::setup::impl$1::from_winit
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":127,"record_offset":22132,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":6079,"source_line":206,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::available_monitors
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21336,"parent_record_offset":16072,"ancestor_record_offset":16072,"inlinee":10702,"source_line":1491,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":22012,"parent_record_offset":21984,"ancestor_record_offset":16072,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":21984,"parent_record_offset":21864,"ancestor_record_offset":16072,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":27616,"parent_record_offset":27592,"ancestor_record_offset":16072,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":21904,"parent_record_offset":21864,"ancestor_record_offset":16072,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":25028,"parent_record_offset":25000,"ancestor_record_offset":16072,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":21864,"parent_record_offset":21832,"ancestor_record_offset":16072,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":25000,"parent_record_offset":24956,"ancestor_record_offset":16072,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":24956,"parent_record_offset":24916,"ancestor_record_offset":16072,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":22096,"parent_record_offset":22068,"ancestor_record_offset":16072,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":17548,"parent_record_offset":17516,"ancestor_record_offset":16072,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":24916,"parent_record_offset":24888,"ancestor_record_offset":16072,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19244,"parent_record_offset":19216,"ancestor_record_offset":16072,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19216,"parent_record_offset":19184,"ancestor_record_offset":16072,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":19152,"parent_record_offset":19120,"ancestor_record_offset":16072,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":21120,"parent_record_offset":20680,"ancestor_record_offset":16072,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19092,"parent_record_offset":18652,"ancestor_record_offset":16072,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19120,"parent_record_offset":19092,"ancestor_record_offset":16072,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":18688,"parent_record_offset":18652,"ancestor_record_offset":16072,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":19184,"parent_record_offset":19120,"ancestor_record_offset":16072,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18744,"parent_record_offset":18716,"ancestor_record_offset":16072,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18716,"parent_record_offset":18688,"ancestor_record_offset":16072,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18848,"parent_record_offset":18776,"ancestor_record_offset":16072,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":18944,"parent_record_offset":18916,"ancestor_record_offset":16072,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::last_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":19324,"parent_record_offset":19292,"ancestor_record_offset":16072,"inlinee":9923,"source_line":2096,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":19356,"parent_record_offset":19292,"ancestor_record_offset":16072,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":24652,"parent_record_offset":24016,"ancestor_record_offset":16072,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":19540,"parent_record_offset":19504,"ancestor_record_offset":16072,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":20680,"parent_record_offset":19540,"ancestor_record_offset":16072,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":17516,"parent_record_offset":17484,"ancestor_record_offset":16072,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":18652,"parent_record_offset":17516,"ancestor_record_offset":16072,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::last_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":19292,"parent_record_offset":18652,"ancestor_record_offset":16072,"inlinee":9922,"source_line":268,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":18356,"parent_record_offset":18328,"ancestor_record_offset":16072,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":18412,"parent_record_offset":18384,"ancestor_record_offset":16072,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":18328,"parent_record_offset":18300,"ancestor_record_offset":16072,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":127,"record_offset":22196,"parent_record_offset":22132,"ancestor_record_offset":16072,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":127,"record_offset":22312,"parent_record_offset":22284,"ancestor_record_offset":16072,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":17764,"parent_record_offset":17736,"ancestor_record_offset":16072,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":17860,"parent_record_offset":17832,"ancestor_record_offset":16072,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":16072,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":127,"record_offset":21448,"parent_record_offset":21420,"ancestor_record_offset":16072,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":127,"record_offset":21524,"parent_record_offset":21496,"ancestor_record_offset":16072,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":127,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":16072,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":16896,"parent_record_offset":16864,"ancestor_record_offset":16072,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":21944,"parent_record_offset":21904,"ancestor_record_offset":16072,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":17980,"parent_record_offset":17952,"ancestor_record_offset":16072,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":16072,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18616,"parent_record_offset":17548,"ancestor_record_offset":16072,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":18916,"parent_record_offset":18716,"ancestor_record_offset":16072,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":19464,"parent_record_offset":17484,"ancestor_record_offset":16072,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":25248,"parent_record_offset":25180,"ancestor_record_offset":16072,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":19036,"parent_record_offset":19008,"ancestor_record_offset":16072,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":21776,"parent_record_offset":21748,"ancestor_record_offset":16072,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":18232,"parent_record_offset":18204,"ancestor_record_offset":16072,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":18140,"parent_record_offset":18112,"ancestor_record_offset":16072,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":16932,"parent_record_offset":16896,"ancestor_record_offset":16072,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":16968,"parent_record_offset":16896,"ancestor_record_offset":16072,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":16424,"parent_record_offset":16356,"ancestor_record_offset":16072,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":18776,"parent_record_offset":18744,"ancestor_record_offset":16072,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":18816,"parent_record_offset":18776,"ancestor_record_offset":16072,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":19008,"parent_record_offset":18972,"ancestor_record_offset":16072,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16728,"parent_record_offset":16696,"ancestor_record_offset":16072,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":17276,"parent_record_offset":17076,"ancestor_record_offset":16072,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":16072,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16356,"parent_record_offset":16328,"ancestor_record_offset":16072,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":17376,"parent_record_offset":17344,"ancestor_record_offset":16072,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16620,"parent_record_offset":16588,"ancestor_record_offset":16072,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16392,"parent_record_offset":16356,"ancestor_record_offset":16072,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":16648,"parent_record_offset":16620,"ancestor_record_offset":16072,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":17304,"parent_record_offset":17276,"ancestor_record_offset":16072,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":17404,"parent_record_offset":17376,"ancestor_record_offset":16072,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":18384,"parent_record_offset":18356,"ancestor_record_offset":16072,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":127,"record_offset":25284,"parent_record_offset":25248,"ancestor_record_offset":16072,"inlinee":10713,"source_line":29,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$16::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":127,"record_offset":25180,"parent_record_offset":25108,"ancestor_record_offset":16072,"inlinee":10709,"source_line":43,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":127,"record_offset":25220,"parent_record_offset":25180,"ancestor_record_offset":16072,"inlinee":10711,"source_line":13,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":18300,"parent_record_offset":18276,"ancestor_record_offset":16072,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":17576,"parent_record_offset":17548,"ancestor_record_offset":16072,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":17604,"parent_record_offset":17576,"ancestor_record_offset":16072,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":18276,"parent_record_offset":18056,"ancestor_record_offset":16072,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18176,"parent_record_offset":18080,"ancestor_record_offset":16072,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18112,"parent_record_offset":18080,"ancestor_record_offset":16072,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18564,"parent_record_offset":17692,"ancestor_record_offset":16072,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":16072,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17664,"parent_record_offset":17636,"ancestor_record_offset":16072,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17636,"parent_record_offset":17604,"ancestor_record_offset":16072,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":17692,"parent_record_offset":17664,"ancestor_record_offset":16072,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17924,"parent_record_offset":17896,"ancestor_record_offset":16072,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17952,"parent_record_offset":17924,"ancestor_record_offset":16072,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":18020,"parent_record_offset":17896,"ancestor_record_offset":16072,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":17896,"parent_record_offset":17692,"ancestor_record_offset":16072,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":18056,"parent_record_offset":17692,"ancestor_record_offset":16072,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":17736,"parent_record_offset":17692,"ancestor_record_offset":16072,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":17832,"parent_record_offset":17692,"ancestor_record_offset":16072,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":18468,"parent_record_offset":17692,"ancestor_record_offset":16072,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":18972,"parent_record_offset":18944,"ancestor_record_offset":16072,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":127,"record_offset":25408,"parent_record_offset":25108,"ancestor_record_offset":16072,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":127,"record_offset":16588,"parent_record_offset":16560,"ancestor_record_offset":16072,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":127,"record_offset":16560,"parent_record_offset":16532,"ancestor_record_offset":16072,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":16464,"parent_record_offset":16292,"ancestor_record_offset":16072,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":17040,"parent_record_offset":17012,"ancestor_record_offset":16072,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":17076,"parent_record_offset":17040,"ancestor_record_offset":16072,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":16696,"parent_record_offset":16496,"ancestor_record_offset":16072,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":16496,"parent_record_offset":16464,"ancestor_record_offset":16072,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":127,"record_offset":16328,"parent_record_offset":16292,"ancestor_record_offset":16072,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":127,"record_offset":17344,"parent_record_offset":17040,"ancestor_record_offset":16072,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":127,"record_offset":16532,"parent_record_offset":16496,"ancestor_record_offset":16072,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::apply_setting_changes::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":21592,"parent_record_offset":21560,"ancestor_record_offset":16072,"inlinee":10705,"source_line":735,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":127,"record_offset":22256,"parent_record_offset":22224,"ancestor_record_offset":16072,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":127,"record_offset":22224,"parent_record_offset":22196,"ancestor_record_offset":16072,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":127,"record_offset":22164,"parent_record_offset":22132,"ancestor_record_offset":16072,"inlinee":6081,"source_line":51,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":21696,"parent_record_offset":21672,"ancestor_record_offset":16072,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":21364,"parent_record_offset":21336,"ancestor_record_offset":16072,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":16072,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::available_monitors::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21392,"parent_record_offset":21364,"ancestor_record_offset":16072,"inlinee":10704,"source_line":1492,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21672,"parent_record_offset":21644,"ancestor_record_offset":16072,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::apply_setting_changes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 731
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1844
// 首条位置: {"module":21,"inlinee":6212}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":34804,"parent_record_offset":34760,"ancestor_record_offset":34760,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":34832,"parent_record_offset":34760,"ancestor_record_offset":34760,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":35132,"parent_record_offset":34760,"ancestor_record_offset":34760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":35284,"parent_record_offset":35260,"ancestor_record_offset":34760,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":35260,"parent_record_offset":35236,"ancestor_record_offset":34760,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":35236,"parent_record_offset":35216,"ancestor_record_offset":34760,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":21,"record_offset":34980,"parent_record_offset":34952,"ancestor_record_offset":34760,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":21,"record_offset":35008,"parent_record_offset":34980,"ancestor_record_offset":34760,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":21,"record_offset":34896,"parent_record_offset":34864,"ancestor_record_offset":34760,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":34924,"parent_record_offset":34896,"ancestor_record_offset":34760,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":34864,"parent_record_offset":34832,"ancestor_record_offset":34760,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":35152,"parent_record_offset":35132,"ancestor_record_offset":34760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":21,"record_offset":35036,"parent_record_offset":35008,"ancestor_record_offset":34760,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":21,"record_offset":35064,"parent_record_offset":35036,"ancestor_record_offset":34760,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::apply_setting_changes::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 735
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x29D1
// 首条位置: {"module":127,"inlinee":10705}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":21592,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":21696,"parent_record_offset":21672,"ancestor_record_offset":21592,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":21644,"parent_record_offset":21616,"ancestor_record_offset":21592,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":21672,"parent_record_offset":21644,"ancestor_record_offset":21592,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::extract_available_monitors_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 849
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":127,"record_offset":125828,"rva":2423360,"va":5371132480}
// 编译实例 RVAs: 0x24fa40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e5300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1486353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8ef51e9ff","size":5,"soft_fail":false,"target_va":5369647872,"va":5371134220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e5980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1484610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8be58e9ff","size":5,"soft_fail":false,"target_va":5369649536,"va":5371134141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140176ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-890210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e89e6af2ff","size":5,"soft_fail":false,"target_va":5370243760,"va":5371133965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402eb4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t636976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e830b80900","size":5,"soft_fail":false,"target_va":5371770016,"va":5371133035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f8220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t688449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e841810a00","size":5,"soft_fail":false,"target_va":5371822624,"va":5371134170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1180910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8ee041200","size":5,"soft_fail":false,"target_va":5372314208,"va":5371133293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140385940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1266874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8ba541300","size":5,"soft_fail":false,"target_va":5372401984,"va":5371135105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403864f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1269646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e88e5f1300","size":5,"soft_fail":false,"target_va":5372404976,"va":5371135325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140394640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1327646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e81e421400","size":5,"soft_fail":false,"target_va":5372462656,"va":5371135005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1616055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8b7a81800","size":5,"soft_fail":false,"target_va":5372749696,"va":5371133636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1616281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e899a91800","size":5,"soft_fail":false,"target_va":5372749712,"va":5371133426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140454870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2115851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e80b492000","size":5,"soft_fail":false,"target_va":5373249648,"va":5371133792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140455590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2120169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8e9592000","size":5,"soft_fail":false,"target_va":5373253008,"va":5371132834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32459741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8dd4bef01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371133614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31843532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8cce4e501","size":5,"soft_fail":false,"target_va":5402978144,"va":5371134607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t171265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8019d0200","size":5,"soft_fail":false,"target_va":5371305600,"va":5371134330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31844254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e89ee7e501","size":5,"soft_fail":false,"target_va":5402978880,"va":5371134621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31844564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8d4e8e501","size":5,"soft_fail":false,"target_va":5402979152,"va":5371134583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::AvailableMonitors::clear
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4279440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8904c4100","size":5,"soft_fail":false,"target_va":5375412256,"va":5371132811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6220518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e8e6ea5e00","size":5,"soft_fail":false,"target_va":5377353776,"va":5371133253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2132749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371132480,"raw_hex":"e80d8b2000","size":5,"soft_fail":false,"target_va":5373265840,"va":5371133086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*40152091(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371132480,"raw_hex":"ff151bac6402","size":6,"soft_fail":false,"va":5371133543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":127,"record_offset":130056,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":8700,"source_line":207,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":127808,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":130308,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":127752,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":126020,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":127,"record_offset":127920,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":127,"record_offset":129972,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":131140,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":130444,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":130112,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":127,"record_offset":128164,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":8686,"source_line":164,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::available_monitors
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":127836,"parent_record_offset":125828,"ancestor_record_offset":125828,"inlinee":10702,"source_line":1491,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":129752,"parent_record_offset":129728,"ancestor_record_offset":125828,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":129776,"parent_record_offset":129752,"ancestor_record_offset":125828,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":128856,"parent_record_offset":128832,"ancestor_record_offset":125828,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":129696,"parent_record_offset":129668,"ancestor_record_offset":125828,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":129728,"parent_record_offset":129696,"ancestor_record_offset":125828,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":128832,"parent_record_offset":128808,"ancestor_record_offset":125828,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":129668,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":127,"record_offset":128808,"parent_record_offset":128784,"ancestor_record_offset":125828,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":127,"record_offset":128692,"parent_record_offset":128644,"ancestor_record_offset":125828,"inlinee":7044,"source_line":1277,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":127,"record_offset":128644,"parent_record_offset":128608,"ancestor_record_offset":125828,"inlinee":7111,"source_line":984,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":127,"record_offset":128576,"parent_record_offset":128552,"ancestor_record_offset":125828,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":127,"record_offset":129204,"parent_record_offset":128932,"ancestor_record_offset":125828,"inlinee":7025,"source_line":1083,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":127,"record_offset":129148,"parent_record_offset":128932,"ancestor_record_offset":125828,"inlinee":7030,"source_line":1644,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":127,"record_offset":128904,"parent_record_offset":128692,"ancestor_record_offset":125828,"inlinee":7045,"source_line":633,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::find_key_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\search.rs
//     evidence={"module":127,"record_offset":128988,"parent_record_offset":128964,"ancestor_record_offset":125828,"inlinee":7039,"source_line":213,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":127,"record_offset":129172,"parent_record_offset":129148,"ancestor_record_offset":125828,"inlinee":7024,"source_line":718,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":127,"record_offset":129020,"parent_record_offset":128988,"ancestor_record_offset":125828,"inlinee":7040,"source_line":384,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_node
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\search.rs
//     evidence={"module":127,"record_offset":128964,"parent_record_offset":128932,"ancestor_record_offset":125828,"inlinee":7038,"source_line":194,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_tree
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\search.rs
//     evidence={"module":127,"record_offset":128932,"parent_record_offset":128692,"ancestor_record_offset":125828,"inlinee":7037,"source_line":48,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":127,"record_offset":128608,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":8697,"source_line":902,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":127,"record_offset":128372,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":8689,"source_line":341,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":127,"record_offset":128552,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":8699,"source_line":1252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":127,"record_offset":128096,"parent_record_offset":127968,"ancestor_record_offset":125828,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":127,"record_offset":127968,"parent_record_offset":127944,"ancestor_record_offset":125828,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":127,"record_offset":128000,"parent_record_offset":127968,"ancestor_record_offset":125828,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":127,"record_offset":128024,"parent_record_offset":128000,"ancestor_record_offset":125828,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":127,"record_offset":127944,"parent_record_offset":127920,"ancestor_record_offset":125828,"inlinee":10715,"source_line":46,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":130744,"parent_record_offset":130716,"ancestor_record_offset":125828,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":130380,"parent_record_offset":130348,"ancestor_record_offset":125828,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":130716,"parent_record_offset":130692,"ancestor_record_offset":125828,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":127,"record_offset":130080,"parent_record_offset":130056,"ancestor_record_offset":125828,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":130348,"parent_record_offset":130308,"ancestor_record_offset":125828,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":129996,"parent_record_offset":129972,"ancestor_record_offset":125828,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":130468,"parent_record_offset":130444,"ancestor_record_offset":125828,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":127776,"parent_record_offset":127752,"ancestor_record_offset":125828,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":126072,"parent_record_offset":126044,"ancestor_record_offset":125828,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":127584,"parent_record_offset":127560,"ancestor_record_offset":125828,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":127560,"parent_record_offset":127536,"ancestor_record_offset":125828,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":127508,"parent_record_offset":127476,"ancestor_record_offset":125828,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":127452,"parent_record_offset":127060,"ancestor_record_offset":125828,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":127476,"parent_record_offset":127452,"ancestor_record_offset":125828,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":127092,"parent_record_offset":127060,"ancestor_record_offset":125828,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":127536,"parent_record_offset":127476,"ancestor_record_offset":125828,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":127140,"parent_record_offset":127116,"ancestor_record_offset":125828,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":127116,"parent_record_offset":127092,"ancestor_record_offset":125828,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":127232,"parent_record_offset":127168,"ancestor_record_offset":125828,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":127320,"parent_record_offset":127296,"ancestor_record_offset":125828,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":127652,"parent_record_offset":127628,"ancestor_record_offset":125828,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":127628,"parent_record_offset":127060,"ancestor_record_offset":125828,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":126044,"parent_record_offset":126020,"ancestor_record_offset":125828,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":127060,"parent_record_offset":126044,"ancestor_record_offset":125828,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":126792,"parent_record_offset":126768,"ancestor_record_offset":125828,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":129636,"parent_record_offset":129612,"ancestor_record_offset":125828,"inlinee":8696,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":126840,"parent_record_offset":126816,"ancestor_record_offset":125828,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":126768,"parent_record_offset":126744,"ancestor_record_offset":125828,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":126260,"parent_record_offset":126236,"ancestor_record_offset":125828,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":126344,"parent_record_offset":126320,"ancestor_record_offset":125828,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":127,"record_offset":128496,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":126284,"parent_record_offset":126260,"ancestor_record_offset":125828,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":129572,"parent_record_offset":129548,"ancestor_record_offset":125828,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":127,"record_offset":129048,"parent_record_offset":128988,"ancestor_record_offset":125828,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":127,"record_offset":129932,"parent_record_offset":129908,"ancestor_record_offset":125828,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":127,"record_offset":130020,"parent_record_offset":129996,"ancestor_record_offset":125828,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":127,"record_offset":129908,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":127,"record_offset":129228,"parent_record_offset":129204,"ancestor_record_offset":125828,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":127,"record_offset":129524,"parent_record_offset":129500,"ancestor_record_offset":125828,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":129500,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":130772,"parent_record_offset":130744,"ancestor_record_offset":125828,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":128048,"parent_record_offset":128024,"ancestor_record_offset":125828,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":126456,"parent_record_offset":126432,"ancestor_record_offset":125828,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":127256,"parent_record_offset":127232,"ancestor_record_offset":125828,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":127028,"parent_record_offset":126072,"ancestor_record_offset":125828,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":127296,"parent_record_offset":127116,"ancestor_record_offset":125828,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":127400,"parent_record_offset":127376,"ancestor_record_offset":125828,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":129252,"parent_record_offset":129228,"ancestor_record_offset":125828,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":128736,"parent_record_offset":128692,"ancestor_record_offset":125828,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":130160,"parent_record_offset":130136,"ancestor_record_offset":125828,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":126680,"parent_record_offset":126656,"ancestor_record_offset":125828,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":126600,"parent_record_offset":126576,"ancestor_record_offset":125828,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":129548,"parent_record_offset":129524,"ancestor_record_offset":125828,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":129104,"parent_record_offset":129072,"ancestor_record_offset":125828,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":128120,"parent_record_offset":128096,"ancestor_record_offset":125828,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":129804,"parent_record_offset":129776,"ancestor_record_offset":125828,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":130412,"parent_record_offset":130308,"ancestor_record_offset":125828,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":127168,"parent_record_offset":127140,"ancestor_record_offset":125828,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":127204,"parent_record_offset":127168,"ancestor_record_offset":125828,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":127376,"parent_record_offset":127344,"ancestor_record_offset":125828,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":127,"record_offset":130136,"parent_record_offset":130112,"ancestor_record_offset":125828,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":129072,"parent_record_offset":129048,"ancestor_record_offset":125828,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":127676,"parent_record_offset":127652,"ancestor_record_offset":125828,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":128436,"parent_record_offset":128400,"ancestor_record_offset":125828,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":127700,"parent_record_offset":127676,"ancestor_record_offset":125828,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":126816,"parent_record_offset":126792,"ancestor_record_offset":125828,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":126744,"parent_record_offset":126720,"ancestor_record_offset":125828,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":126096,"parent_record_offset":126072,"ancestor_record_offset":125828,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":126120,"parent_record_offset":126096,"ancestor_record_offset":125828,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":126720,"parent_record_offset":126524,"ancestor_record_offset":125828,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":126632,"parent_record_offset":126548,"ancestor_record_offset":125828,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":126576,"parent_record_offset":126548,"ancestor_record_offset":125828,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":126976,"parent_record_offset":126196,"ancestor_record_offset":125828,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":126548,"parent_record_offset":126524,"ancestor_record_offset":125828,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":126172,"parent_record_offset":126148,"ancestor_record_offset":125828,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":126148,"parent_record_offset":126120,"ancestor_record_offset":125828,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":126196,"parent_record_offset":126172,"ancestor_record_offset":125828,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":126404,"parent_record_offset":126376,"ancestor_record_offset":125828,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":126432,"parent_record_offset":126404,"ancestor_record_offset":125828,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":126492,"parent_record_offset":126376,"ancestor_record_offset":125828,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":126376,"parent_record_offset":126196,"ancestor_record_offset":125828,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":126524,"parent_record_offset":126196,"ancestor_record_offset":125828,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":126236,"parent_record_offset":126196,"ancestor_record_offset":125828,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":126320,"parent_record_offset":126196,"ancestor_record_offset":125828,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":126892,"parent_record_offset":126196,"ancestor_record_offset":125828,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":127,"record_offset":128524,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":127,"record_offset":128400,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":127344,"parent_record_offset":127320,"ancestor_record_offset":125828,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":127,"record_offset":128200,"parent_record_offset":128164,"ancestor_record_offset":125828,"inlinee":8687,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::util::has_flag
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/util.rs
//     evidence={"module":127,"record_offset":129612,"parent_record_offset":128200,"ancestor_record_offset":125828,"inlinee":8694,"source_line":53,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":127,"record_offset":127860,"parent_record_offset":127836,"ancestor_record_offset":125828,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::available_monitors::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":127,"record_offset":127884,"parent_record_offset":127860,"ancestor_record_offset":125828,"inlinee":10704,"source_line":1492,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::on_cursor_moved_absolute
// 记录形式: compiled_procedure; instances=1
// 源码行: 873
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":127,"record_offset":131888,"rva":2426400,"va":5371135520}
// 编译实例 RVAs: 0x250620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t171084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371135520,"raw_hex":"e84c9c0200","size":5,"soft_fail":false,"target_va":5371306912,"va":5371135823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31842977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371135520,"raw_hex":"e8a1e2e501","size":5,"soft_fail":false,"target_va":5402978880,"va":5371135898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":133764,"parent_record_offset":131888,"ancestor_record_offset":131888,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":133820,"parent_record_offset":131888,"ancestor_record_offset":131888,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":132032,"parent_record_offset":131888,"ancestor_record_offset":131888,"inlinee":7122,"source_line":1593,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":133996,"parent_record_offset":131888,"ancestor_record_offset":131888,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":133928,"parent_record_offset":133896,"ancestor_record_offset":131888,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":133896,"parent_record_offset":133856,"ancestor_record_offset":131888,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":133856,"parent_record_offset":133820,"ancestor_record_offset":131888,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":127,"record_offset":133788,"parent_record_offset":133764,"ancestor_record_offset":131888,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":132084,"parent_record_offset":132056,"ancestor_record_offset":131888,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":133596,"parent_record_offset":133572,"ancestor_record_offset":131888,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":133572,"parent_record_offset":133548,"ancestor_record_offset":131888,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":133520,"parent_record_offset":133488,"ancestor_record_offset":131888,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":133464,"parent_record_offset":133072,"ancestor_record_offset":131888,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":133488,"parent_record_offset":133464,"ancestor_record_offset":131888,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":133104,"parent_record_offset":133072,"ancestor_record_offset":131888,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":133548,"parent_record_offset":133488,"ancestor_record_offset":131888,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":133152,"parent_record_offset":133128,"ancestor_record_offset":131888,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":133128,"parent_record_offset":133104,"ancestor_record_offset":131888,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":133244,"parent_record_offset":133180,"ancestor_record_offset":131888,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":133332,"parent_record_offset":133308,"ancestor_record_offset":131888,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":133664,"parent_record_offset":133640,"ancestor_record_offset":131888,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":133640,"parent_record_offset":133072,"ancestor_record_offset":131888,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":132056,"parent_record_offset":132032,"ancestor_record_offset":131888,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":133072,"parent_record_offset":132056,"ancestor_record_offset":131888,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":132804,"parent_record_offset":132780,"ancestor_record_offset":131888,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":132852,"parent_record_offset":132828,"ancestor_record_offset":131888,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":132780,"parent_record_offset":132756,"ancestor_record_offset":131888,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":132272,"parent_record_offset":132248,"ancestor_record_offset":131888,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":132356,"parent_record_offset":132332,"ancestor_record_offset":131888,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":132296,"parent_record_offset":132272,"ancestor_record_offset":131888,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":132468,"parent_record_offset":132444,"ancestor_record_offset":131888,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":133268,"parent_record_offset":133244,"ancestor_record_offset":131888,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":133040,"parent_record_offset":132084,"ancestor_record_offset":131888,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":133308,"parent_record_offset":133128,"ancestor_record_offset":131888,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":133412,"parent_record_offset":133388,"ancestor_record_offset":131888,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":132692,"parent_record_offset":132668,"ancestor_record_offset":131888,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":132612,"parent_record_offset":132588,"ancestor_record_offset":131888,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":133960,"parent_record_offset":133856,"ancestor_record_offset":131888,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":133180,"parent_record_offset":133152,"ancestor_record_offset":131888,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":133216,"parent_record_offset":133180,"ancestor_record_offset":131888,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":133388,"parent_record_offset":133356,"ancestor_record_offset":131888,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":133688,"parent_record_offset":133664,"ancestor_record_offset":131888,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":133712,"parent_record_offset":133688,"ancestor_record_offset":131888,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":132828,"parent_record_offset":132804,"ancestor_record_offset":131888,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":132756,"parent_record_offset":132732,"ancestor_record_offset":131888,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":132108,"parent_record_offset":132084,"ancestor_record_offset":131888,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":132132,"parent_record_offset":132108,"ancestor_record_offset":131888,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":132732,"parent_record_offset":132536,"ancestor_record_offset":131888,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":132644,"parent_record_offset":132560,"ancestor_record_offset":131888,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":132588,"parent_record_offset":132560,"ancestor_record_offset":131888,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":132988,"parent_record_offset":132208,"ancestor_record_offset":131888,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":132560,"parent_record_offset":132536,"ancestor_record_offset":131888,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":132184,"parent_record_offset":132160,"ancestor_record_offset":131888,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":132160,"parent_record_offset":132132,"ancestor_record_offset":131888,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":132208,"parent_record_offset":132184,"ancestor_record_offset":131888,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":132416,"parent_record_offset":132388,"ancestor_record_offset":131888,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":132444,"parent_record_offset":132416,"ancestor_record_offset":131888,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":132504,"parent_record_offset":132388,"ancestor_record_offset":131888,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":132388,"parent_record_offset":132208,"ancestor_record_offset":131888,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":132536,"parent_record_offset":132208,"ancestor_record_offset":131888,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":132248,"parent_record_offset":132208,"ancestor_record_offset":131888,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":132332,"parent_record_offset":132208,"ancestor_record_offset":131888,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":132904,"parent_record_offset":132208,"ancestor_record_offset":131888,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":133356,"parent_record_offset":133332,"ancestor_record_offset":131888,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::shut_down_everything
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 879
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x297A
// 首条位置: {"module":127,"inlinee":10618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":8316,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":8344,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":5558,"source_line":1575,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::remove_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":127,"record_offset":11432,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":10621,"source_line":1365,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":14200,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":127,"record_offset":14228,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":14500,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":11404,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":14556,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":127,"record_offset":14256,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":127,"record_offset":14116,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::with_global_allocator
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":127,"record_offset":13444,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":10629,"source_line":684,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":9908,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":10619,"source_line":2217,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":127,"record_offset":10740,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":8216,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":8216,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":8216,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10104,"parent_record_offset":10076,"ancestor_record_offset":8216,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":127,"record_offset":10488,"parent_record_offset":10432,"ancestor_record_offset":8216,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10460,"parent_record_offset":10432,"ancestor_record_offset":8216,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10076,"parent_record_offset":10048,"ancestor_record_offset":8216,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10048,"parent_record_offset":10020,"ancestor_record_offset":8216,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10224,"parent_record_offset":10048,"ancestor_record_offset":8216,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":127,"record_offset":10432,"parent_record_offset":10404,"ancestor_record_offset":8216,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":14368,"parent_record_offset":14332,"ancestor_record_offset":8216,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":13476,"parent_record_offset":13444,"ancestor_record_offset":8216,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":127,"record_offset":14332,"parent_record_offset":14308,"ancestor_record_offset":8216,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":9992,"parent_record_offset":9944,"ancestor_record_offset":8216,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":127,"record_offset":10020,"parent_record_offset":9992,"ancestor_record_offset":8216,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":127,"record_offset":8404,"parent_record_offset":8372,"ancestor_record_offset":8216,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":9860,"parent_record_offset":9828,"ancestor_record_offset":8216,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":127,"record_offset":12940,"parent_record_offset":12848,"ancestor_record_offset":8216,"inlinee":10627,"source_line":294,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":9804,"parent_record_offset":9416,"ancestor_record_offset":8216,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":8216,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":12848,"parent_record_offset":11432,"ancestor_record_offset":8216,"inlinee":10626,"source_line":210,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":9444,"parent_record_offset":9416,"ancestor_record_offset":8216,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":127,"record_offset":12476,"parent_record_offset":11432,"ancestor_record_offset":8216,"inlinee":10622,"source_line":297,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":9492,"parent_record_offset":9468,"ancestor_record_offset":8216,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":8216,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":12512,"parent_record_offset":12476,"ancestor_record_offset":8216,"inlinee":10623,"source_line":379,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":9584,"parent_record_offset":9520,"ancestor_record_offset":8216,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":9672,"parent_record_offset":9648,"ancestor_record_offset":8216,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":127,"record_offset":12716,"parent_record_offset":12692,"ancestor_record_offset":8216,"inlinee":10625,"source_line":381,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":8372,"parent_record_offset":8344,"ancestor_record_offset":8216,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":127,"record_offset":9416,"parent_record_offset":8372,"ancestor_record_offset":8216,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":127,"record_offset":13356,"parent_record_offset":11432,"ancestor_record_offset":8216,"inlinee":10628,"source_line":428,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":127,"record_offset":9148,"parent_record_offset":9124,"ancestor_record_offset":8216,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":127,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":8216,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":8216,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":8216,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":127,"record_offset":10776,"parent_record_offset":10740,"ancestor_record_offset":8216,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":8216,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":127,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":8216,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":127,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":8216,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":127,"record_offset":13084,"parent_record_offset":13012,"ancestor_record_offset":8216,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":127,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":8216,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":8216,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":9384,"parent_record_offset":8404,"ancestor_record_offset":8216,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":9648,"parent_record_offset":9468,"ancestor_record_offset":8216,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":127,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":8216,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":10380,"parent_record_offset":9944,"ancestor_record_offset":8216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":8216,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":8216,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":13380,"parent_record_offset":13356,"ancestor_record_offset":8216,"inlinee":6972,"source_line":1335,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":8216,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":14400,"parent_record_offset":14368,"ancestor_record_offset":8216,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":13160,"parent_record_offset":13084,"ancestor_record_offset":8216,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":8216,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":12980,"parent_record_offset":12940,"ancestor_record_offset":8216,"inlinee":8889,"source_line":1002,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_simple_untyped
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":13012,"parent_record_offset":12980,"ancestor_record_offset":8216,"inlinee":8890,"source_line":1059,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":13248,"parent_record_offset":13084,"ancestor_record_offset":8216,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":13612,"parent_record_offset":13528,"ancestor_record_offset":8216,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":13636,"parent_record_offset":13612,"ancestor_record_offset":8216,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":9520,"parent_record_offset":9492,"ancestor_record_offset":8216,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":9556,"parent_record_offset":9520,"ancestor_record_offset":8216,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":9728,"parent_record_offset":9696,"ancestor_record_offset":8216,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":12772,"parent_record_offset":12740,"ancestor_record_offset":8216,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13840,"parent_record_offset":13808,"ancestor_record_offset":8216,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13556,"parent_record_offset":13528,"ancestor_record_offset":8216,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":8216,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":8216,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":8216,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":14164,"parent_record_offset":14140,"ancestor_record_offset":8216,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":14428,"parent_record_offset":14368,"ancestor_record_offset":8216,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":14460,"parent_record_offset":14332,"ancestor_record_offset":8216,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":127,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":8216,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":127,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":8216,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":8432,"parent_record_offset":8404,"ancestor_record_offset":8216,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":8216,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":127,"record_offset":9076,"parent_record_offset":8876,"ancestor_record_offset":8216,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8988,"parent_record_offset":8904,"ancestor_record_offset":8216,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8932,"parent_record_offset":8904,"ancestor_record_offset":8216,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":9332,"parent_record_offset":8540,"ancestor_record_offset":8216,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8904,"parent_record_offset":8876,"ancestor_record_offset":8216,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":8216,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8492,"parent_record_offset":8460,"ancestor_record_offset":8216,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":8216,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":8756,"parent_record_offset":8728,"ancestor_record_offset":8216,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":8784,"parent_record_offset":8756,"ancestor_record_offset":8216,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":8844,"parent_record_offset":8728,"ancestor_record_offset":8216,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":127,"record_offset":8728,"parent_record_offset":8540,"ancestor_record_offset":8216,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":127,"record_offset":8876,"parent_record_offset":8540,"ancestor_record_offset":8216,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":8588,"parent_record_offset":8540,"ancestor_record_offset":8216,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":8672,"parent_record_offset":8540,"ancestor_record_offset":8216,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":127,"record_offset":9248,"parent_record_offset":8540,"ancestor_record_offset":8216,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13504,"parent_record_offset":13444,"ancestor_record_offset":8216,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13676,"parent_record_offset":13444,"ancestor_record_offset":8216,"inlinee":10631,"source_line":639,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":127,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":8216,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":127,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":8216,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":127,"record_offset":13528,"parent_record_offset":13504,"ancestor_record_offset":8216,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":127,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":8216,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":127,"record_offset":10348,"parent_record_offset":10264,"ancestor_record_offset":8216,"inlinee":5032,"source_line":499,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":10292,"parent_record_offset":10264,"ancestor_record_offset":8216,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":8216,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":9944,"parent_record_offset":9908,"ancestor_record_offset":8216,"inlinee":10184,"source_line":2185,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":127,"record_offset":10264,"parent_record_offset":9944,"ancestor_record_offset":8216,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":127,"record_offset":10888,"parent_record_offset":10860,"ancestor_record_offset":8216,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":127,"record_offset":10944,"parent_record_offset":10916,"ancestor_record_offset":8216,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":127,"record_offset":10916,"parent_record_offset":10888,"ancestor_record_offset":8216,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":127,"record_offset":10860,"parent_record_offset":10832,"ancestor_record_offset":8216,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":127,"record_offset":10832,"parent_record_offset":10804,"ancestor_record_offset":8216,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":127,"record_offset":10804,"parent_record_offset":10776,"ancestor_record_offset":8216,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::shut_down_everything::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":127,"record_offset":13732,"parent_record_offset":13444,"ancestor_record_offset":8216,"inlinee":10633,"source_line":893,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::shut_down_everything::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 893
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2989
// 首条位置: {"module":127,"inlinee":10633}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::next_request_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 935
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2992
// 首条位置: {"module":127,"inlinee":10642}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::on_confirmation
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 940
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x29A6
// 首条位置: {"module":127,"inlinee":10662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 944
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x299A
// 首条位置: {"module":127,"inlinee":10650}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65808,"parent_record_offset":65784,"ancestor_record_offset":65784,"inlinee":8949,"source_line":1221,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":65784,"inlinee":10653,"source_line":1549,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65880,"parent_record_offset":65856,"ancestor_record_offset":65784,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::event_loop::impl$5::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":127,"record_offset":65832,"parent_record_offset":65808,"ancestor_record_offset":65784,"inlinee":10652,"source_line":72,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$3::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 952
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":127,"record_offset":134028,"rva":2426784,"va":5371135904}
// 编译实例 RVAs: 0x2507a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x1403b71b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1468936","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371135904,"raw_hex":"e9086a1600","size":5,"soft_fail":false,"target_va":5372604848,"va":5371135907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":127,"record_offset":134148,"parent_record_offset":134028,"ancestor_record_offset":134028,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::gather_pending_video_setting_changes
// 记录形式: compiled_procedure; instances=1
// 源码行: 957
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":127,"record_offset":134180,"rva":2426800,"va":5371135920}
// 编译实例 RVAs: 0x2507b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1112942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371135920,"raw_hex":"e86efb1000","size":5,"soft_fail":false,"target_va":5372249392,"va":5371136445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31843738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371135920,"raw_hex":"e89ae5e501","size":5,"soft_fail":false,"target_va":5402980128,"va":5371136385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31839059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371135920,"raw_hex":"e853d3e501","size":5,"soft_fail":false,"target_va":5402975328,"va":5371136264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31839201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371135920,"raw_hex":"e8e1d3e501","size":5,"soft_fail":false,"target_va":5402975504,"va":5371136298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31839557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371135920,"raw_hex":"e845d5e501","size":5,"soft_fail":false,"target_va":5402975696,"va":5371136134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":134344,"parent_record_offset":134180,"ancestor_record_offset":134180,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":134884,"parent_record_offset":134180,"ancestor_record_offset":134180,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":127,"record_offset":135236,"parent_record_offset":134180,"ancestor_record_offset":134180,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":136136,"parent_record_offset":134180,"ancestor_record_offset":134180,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":135636,"parent_record_offset":134180,"ancestor_record_offset":134180,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::impl$25::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":127,"record_offset":136180,"parent_record_offset":134180,"ancestor_record_offset":134180,"inlinee":10686,"source_line":739,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":134432,"parent_record_offset":134408,"ancestor_record_offset":134180,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":134408,"parent_record_offset":134368,"ancestor_record_offset":134180,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":127,"record_offset":134916,"parent_record_offset":134884,"ancestor_record_offset":134180,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":127,"record_offset":134368,"parent_record_offset":134344,"ancestor_record_offset":134180,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":127,"record_offset":134984,"parent_record_offset":134952,"ancestor_record_offset":134180,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":127,"record_offset":134952,"parent_record_offset":134916,"ancestor_record_offset":134180,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":127,"record_offset":135080,"parent_record_offset":135056,"ancestor_record_offset":134180,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":127,"record_offset":134476,"parent_record_offset":134432,"ancestor_record_offset":134180,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":135016,"parent_record_offset":134984,"ancestor_record_offset":134180,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":127,"record_offset":135056,"parent_record_offset":134952,"ancestor_record_offset":134180,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":127,"record_offset":134776,"parent_record_offset":134752,"ancestor_record_offset":134180,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":127,"record_offset":135180,"parent_record_offset":135156,"ancestor_record_offset":134180,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":127,"record_offset":135764,"parent_record_offset":135704,"ancestor_record_offset":134180,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":134540,"parent_record_offset":134432,"ancestor_record_offset":134180,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":127,"record_offset":134728,"parent_record_offset":134432,"ancestor_record_offset":134180,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":134600,"parent_record_offset":134576,"ancestor_record_offset":134180,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":127,"record_offset":134576,"parent_record_offset":134540,"ancestor_record_offset":134180,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":127,"record_offset":134752,"parent_record_offset":134728,"ancestor_record_offset":134180,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":135156,"parent_record_offset":135132,"ancestor_record_offset":134180,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":127,"record_offset":135132,"parent_record_offset":135080,"ancestor_record_offset":134180,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":136056,"parent_record_offset":136028,"ancestor_record_offset":134180,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":135568,"parent_record_offset":135396,"ancestor_record_offset":134180,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":135704,"parent_record_offset":135672,"ancestor_record_offset":134180,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":135336,"parent_record_offset":135304,"ancestor_record_offset":134180,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":135500,"parent_record_offset":135472,"ancestor_record_offset":134180,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":135736,"parent_record_offset":135704,"ancestor_record_offset":134180,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":135524,"parent_record_offset":135500,"ancestor_record_offset":134180,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":135592,"parent_record_offset":135568,"ancestor_record_offset":134180,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":127,"record_offset":135360,"parent_record_offset":135336,"ancestor_record_offset":134180,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":127,"record_offset":135472,"parent_record_offset":135448,"ancestor_record_offset":134180,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":127,"record_offset":135448,"parent_record_offset":135424,"ancestor_record_offset":134180,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":135800,"parent_record_offset":135636,"ancestor_record_offset":134180,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":127,"record_offset":135268,"parent_record_offset":135236,"ancestor_record_offset":134180,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":135396,"parent_record_offset":135268,"ancestor_record_offset":134180,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":136028,"parent_record_offset":135828,"ancestor_record_offset":134180,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":127,"record_offset":135828,"parent_record_offset":135800,"ancestor_record_offset":134180,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":127,"record_offset":135672,"parent_record_offset":135636,"ancestor_record_offset":134180,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":127,"record_offset":135304,"parent_record_offset":135268,"ancestor_record_offset":134180,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":127,"record_offset":135424,"parent_record_offset":135396,"ancestor_record_offset":134180,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00127.jsonl.gz"}
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
// 完整结构及原始类型记录: game_loop.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::CursorRequestClient
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::GameLoop
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 3
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::GameLoopPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: game_loop.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: game_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
// 信息 ID: eb44fe961eae1c30a4dfc05a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=65784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::next_request_id
// 信息 ID: 35df7b2994a783489a82b3d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=49760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::on_confirmation
// 信息 ID: 842c2e8ad13c147f74cbd214
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=71276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::apply_setting_changes
// 信息 ID: a9cb5ac166b6a0a54bfb57e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=16072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::extract_available_monitors_info
// 信息 ID: f029cfecd928141711e208fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x246a96 -> 0x24fa40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::main_loop
// 信息 ID: e24a08dc3c2c832e80998374
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::run @ 0x245de2 -> 0x2469f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::new
// 信息 ID: 661784f0571797a64600877c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=122456; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=147184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::process_app_event
// 信息 ID: 3c3bdb9440dc0a02772fa627
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=53080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::receive_app_events
// 信息 ID: 09045e75619e2a55d28b5498
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=47936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::run
// 信息 ID: b4aee797a195cab3cdaef64f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3d3999 -> 0x245db0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x3d4ea9 -> 0x245db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::send_window_events
// 信息 ID: cc5e4273929a765838cb96e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=48172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::simulate_and_render_frame
// 信息 ID: 9b1f45a52520ebd9e79f47c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=92356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::gather_pending_video_setting_changes
// 信息 ID: 465032c2ec154de6c02e7cd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7a139ff9a9c5c2be0a13d0f3: void (*)(bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>),void (*)(bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)> > @ 0xd569c -> 0x2507b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>),void (*)(bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)> @ 0x35a206 -> 0x2507b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::apply_setting_changes::closure$0
// 信息 ID: dbbf75783cc6011fd83653f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::impl$0::apply_setting_changes", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::game_loop::impl$0::apply_setting_changes::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::game_loop::impl$0::apply_setting_changes::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::game_loop::impl$0::apply_setting_changes::closure_env$0 <- alloc::collections::vec_deque::into_iter::impl$2::try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::game_loop::impl$0::apply_setting_changes::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::game_loop::impl$0::apply_setting_changes::closure_env$0> > >; module=21, inline_record=34760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::apply_setting_changes::closure$1
// 信息 ID: 8fad9bc2f6e7205fd20024fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::game_loop::impl$0::apply_setting_changes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=21592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::main_loop::closure$0
// 信息 ID: 91f082bf2eefed95fd18742e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::impl$0::main_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=29640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::main_loop::closure$1
// 信息 ID: 4a5e282508b9e2f7500a93c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::game_loop::impl$0::main_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=33428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::process_app_event::closure$0
// 信息 ID: 79667a58d452348e12775931
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::impl$0::process_app_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=86496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$0
// 信息 ID: 0d2cd6fd4ecb20a0dcc9d85f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::impl$0::simulate_and_render_frame", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=113824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$3::build
// 信息 ID: 1aad33f53e820f4b4b2f47fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::on_cursor_moved_absolute
// 信息 ID: babf1bcb704c07d87cb3a532
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x248224 -> 0x250620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::shut_down_everything
// 信息 ID: 883363e062b5f6068914705a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::run; module=127, inline_record=8216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::shut_down_everything::closure$0
// 信息 ID: 866b321595356bfa7fb7b2d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::shut_down_everything", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::run; module=127, inline_record=13732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
