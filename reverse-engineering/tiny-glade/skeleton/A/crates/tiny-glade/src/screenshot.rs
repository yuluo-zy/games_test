// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/screenshot.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ad6c29af47a7befdbca20e34eb0796faf063cdceea273145067e36ce6a8368d4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: screenshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 14, 71, 73, 102, 197, 223, 234
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"PDB_registry_path_only","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"PDB_registry_path_only","version":"1.19.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use steamworks; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":102,"record_offset":5356,"rva":1412944,"va":5370122064}
// 编译实例 RVAs: 0x158f50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403482a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2028334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122064,"raw_hex":"e82ef31e00","size":5,"soft_fail":false,"target_va":5372150432,"va":5370122093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b7630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2483891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122064,"raw_hex":"e8b3e62500","size":5,"soft_fail":false,"target_va":5372606000,"va":5370122104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1403b7030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2482338","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370122064,"raw_hex":"e9a2e02500","size":5,"soft_fail":false,"target_va":5372604464,"va":5370122121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":102,"record_offset":5480,"parent_record_offset":5356,"ancestor_record_offset":5356,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25FD
// 首条位置: {"module":102,"inlinee":9725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$10::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2609
// 首条位置: {"module":102,"inlinee":9737}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":38520,"parent_record_offset":38452,"ancestor_record_offset":38452,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":38604,"parent_record_offset":38452,"ancestor_record_offset":38452,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":39016,"parent_record_offset":38992,"ancestor_record_offset":38452,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":39040,"parent_record_offset":39016,"ancestor_record_offset":38452,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":38992,"parent_record_offset":38956,"ancestor_record_offset":38452,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":38956,"parent_record_offset":38928,"ancestor_record_offset":38452,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":38928,"parent_record_offset":38900,"ancestor_record_offset":38452,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":39112,"parent_record_offset":38928,"ancestor_record_offset":38452,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38860,"parent_record_offset":38824,"ancestor_record_offset":38452,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38824,"parent_record_offset":38788,"ancestor_record_offset":38452,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38788,"parent_record_offset":38752,"ancestor_record_offset":38452,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":38900,"parent_record_offset":38860,"ancestor_record_offset":38452,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":38752,"parent_record_offset":38712,"ancestor_record_offset":38452,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":102,"record_offset":39224,"parent_record_offset":38712,"ancestor_record_offset":38452,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":102,"record_offset":39176,"parent_record_offset":39152,"ancestor_record_offset":38452,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":39152,"parent_record_offset":38860,"ancestor_record_offset":38452,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":39068,"parent_record_offset":39040,"ancestor_record_offset":38452,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":38544,"parent_record_offset":38520,"ancestor_record_offset":38452,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":38568,"parent_record_offset":38544,"ancestor_record_offset":38452,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":38640,"parent_record_offset":38604,"ancestor_record_offset":38452,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":102,"record_offset":38676,"parent_record_offset":38640,"ancestor_record_offset":38452,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":102,"record_offset":38712,"parent_record_offset":38676,"ancestor_record_offset":38452,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$1::drop
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3469
// 首条位置: {"module":234,"inlinee":13417}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":234,"record_offset":770148,"parent_record_offset":769944,"ancestor_record_offset":769944,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":234,"record_offset":769976,"parent_record_offset":769944,"ancestor_record_offset":769944,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":234,"record_offset":770004,"parent_record_offset":769944,"ancestor_record_offset":769944,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":770080,"parent_record_offset":770052,"ancestor_record_offset":769944,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":770104,"parent_record_offset":770080,"ancestor_record_offset":769944,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":234,"record_offset":770052,"parent_record_offset":770028,"ancestor_record_offset":769944,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":234,"record_offset":770028,"parent_record_offset":770004,"ancestor_record_offset":769944,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::ScreenshotCapturedCallback::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25FB
// 首条位置: {"module":102,"inlinee":9723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":11904,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":11868,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":12020,"parent_record_offset":11996,"ancestor_record_offset":11868,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":11940,"parent_record_offset":11904,"ancestor_record_offset":11868,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":11972,"parent_record_offset":11940,"ancestor_record_offset":11868,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":12048,"parent_record_offset":12020,"ancestor_record_offset":11868,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::ScreenshotCapturedCallback::call
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2606
// 首条位置: {"module":102,"inlinee":9734}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$47::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":35164,"parent_record_offset":34996,"ancestor_record_offset":34996,"inlinee":5521,"source_line":2019,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":35044,"parent_record_offset":34996,"ancestor_record_offset":34996,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":35128,"parent_record_offset":34996,"ancestor_record_offset":34996,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":34996,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":35276,"parent_record_offset":35192,"ancestor_record_offset":34996,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":35192,"parent_record_offset":35164,"ancestor_record_offset":34996,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":102,"record_offset":35220,"parent_record_offset":35192,"ancestor_record_offset":34996,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":35068,"parent_record_offset":35044,"ancestor_record_offset":34996,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":35244,"parent_record_offset":35220,"ancestor_record_offset":34996,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":35092,"parent_record_offset":35068,"ancestor_record_offset":34996,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests
// 记录形式: compiled_procedure; instances=1
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":102,"record_offset":5540,"rva":1413008,"va":5370122128}
// 编译实例 RVAs: 0x158f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1404714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8d690eaff","size":5,"soft_fail":false,"target_va":5368720672,"va":5370125381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14002a990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1243659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8f505edff","size":5,"soft_fail":false,"target_va":5368883600,"va":5370127254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dfa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-497721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8c767f8ff","size":5,"soft_fail":false,"target_va":5369625088,"va":5370122804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce7b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1526644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8744b1700","size":5,"soft_fail":false,"target_va":5371652016,"va":5370125367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cea10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1525089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e861451700","size":5,"soft_fail":false,"target_va":5371652624,"va":5370127530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0070
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1535653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8a56e1700","size":5,"soft_fail":false,"target_va":5371658352,"va":5370122694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0940
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1535118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e88e6c1700","size":5,"soft_fail":false,"target_va":5371660608,"va":5370125485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403481a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2021492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e874d81e00","size":5,"soft_fail":false,"target_va":5372150176,"va":5370128679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2106450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e852242000","size":5,"soft_fail":false,"target_va":5372236016,"va":5370129561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140369d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2159220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e874f22000","size":5,"soft_fail":false,"target_va":5372288272,"va":5370129047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036ea40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2179595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e80b422100","size":5,"soft_fail":false,"target_va":5372308032,"va":5370128432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140381680
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2256036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8a46c2200","size":5,"soft_fail":false,"target_va":5372384896,"va":5370128855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403857c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2270788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e844a62200","size":5,"soft_fail":false,"target_va":5372401600,"va":5370130807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038ea50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2309134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e80e3c2300","size":5,"soft_fail":false,"target_va":5372439120,"va":5370129981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2626224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8b0122800","size":5,"soft_fail":false,"target_va":5372749696,"va":5370123467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2626442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e88a132800","size":5,"soft_fail":false,"target_va":5372749712,"va":5370123265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4273371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8db344100","size":5,"soft_fail":false,"target_va":5374398944,"va":5370125568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5047927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e877064d00","size":5,"soft_fail":false,"target_va":5375171056,"va":5370123124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6114522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8da4c5d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5370125393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7312776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e888956f00","size":5,"soft_fail":false,"target_va":5377436496,"va":5370123715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33469424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8f0b3fe01","size":5,"soft_fail":false,"target_va":5403593360,"va":5370123931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33465631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e81fa5fe01","size":5,"soft_fail":false,"target_va":5403593376,"va":5370127740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t32850311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e88741f501","size":5,"soft_fail":false,"target_va":5402978144,"va":5370127828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7406642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e832047100","size":5,"soft_fail":false,"target_va":5377529888,"va":5370123241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32793344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e80063f401","size":5,"soft_fail":false,"target_va":5402920512,"va":5370127163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t1181479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e827071200","size":5,"soft_fail":false,"target_va":5371305056,"va":5370123572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7399212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e82ce77000","size":5,"soft_fail":false,"target_va":5377527120,"va":5370127903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4660398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8ae1c4700","size":5,"soft_fail":false,"target_va":5374783040,"va":5370122637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::consume
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5188424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8482b4f00","size":5,"soft_fail":false,"target_va":5375315840,"va":5370127411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::peek
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5188515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8a32b4f00","size":5,"soft_fail":false,"target_va":5375315792,"va":5370127272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4274718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e81e3a4100","size":5,"soft_fail":false,"target_va":5374399200,"va":5370124477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32841913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8b920f501","size":5,"soft_fail":false,"target_va":5402968992,"va":5370127074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t32843010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e80225f501","size":5,"soft_fail":false,"target_va":5402970800,"va":5370127785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t32841597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e87d1ff501","size":5,"soft_fail":false,"target_va":5402963904,"va":5370122302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t32842957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8cd24f501","size":5,"soft_fail":false,"target_va":5402965344,"va":5370122382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6030822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8e6055c00","size":5,"soft_fail":false,"target_va":5376153904,"va":5370123077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7316656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8b0a46f00","size":5,"soft_fail":false,"target_va":5377439776,"va":5370123115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6821861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370122128,"raw_hex":"e8e5176800","size":5,"soft_fail":false,"target_va":5376945008,"va":5370123142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":102,"record_offset":9080,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":12880,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":10576,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":10104,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":10304,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":5768,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":10664,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":17116,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":17172,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":102,"record_offset":8672,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":7036,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":12692,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":102,"record_offset":11636,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":102,"record_offset":16608,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":102,"record_offset":7584,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":9164,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":16944,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":10132,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":8912,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":10972,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":8940,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":11868,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":9723,"source_line":97,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::add_captured_screenshots_to_steam_library
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":8020,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":9719,"source_line":450,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::consume_steam_screenshot_request
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":5796,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":9707,"source_line":431,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":5540,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":5540,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":5540,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":10412,"parent_record_offset":10376,"ancestor_record_offset":5540,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":10444,"parent_record_offset":10412,"ancestor_record_offset":5540,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":9296,"parent_record_offset":9240,"ancestor_record_offset":5540,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":10376,"parent_record_offset":10340,"ancestor_record_offset":5540,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":102,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":5540,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":17380,"parent_record_offset":17356,"ancestor_record_offset":5540,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":9268,"parent_record_offset":9240,"ancestor_record_offset":5540,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":17356,"parent_record_offset":17328,"ancestor_record_offset":5540,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":5540,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":5540,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":11324,"parent_record_offset":11296,"ancestor_record_offset":5540,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":5540,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":11480,"parent_record_offset":11296,"ancestor_record_offset":5540,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":9240,"parent_record_offset":9212,"ancestor_record_offset":5540,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":11228,"parent_record_offset":11192,"ancestor_record_offset":5540,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":11192,"parent_record_offset":11156,"ancestor_record_offset":5540,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":11156,"parent_record_offset":11120,"ancestor_record_offset":5540,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":13936,"parent_record_offset":13900,"ancestor_record_offset":5540,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":10340,"parent_record_offset":10304,"ancestor_record_offset":5540,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":5856,"parent_record_offset":5796,"ancestor_record_offset":5540,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":5540,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":10704,"parent_record_offset":10664,"ancestor_record_offset":5540,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":17264,"parent_record_offset":17232,"ancestor_record_offset":5540,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":17304,"parent_record_offset":17264,"ancestor_record_offset":5540,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":6468,"parent_record_offset":6380,"ancestor_record_offset":5540,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":6380,"parent_record_offset":5796,"ancestor_record_offset":5540,"inlinee":9718,"source_line":1487,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":11268,"parent_record_offset":11228,"ancestor_record_offset":5540,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":11120,"parent_record_offset":11080,"ancestor_record_offset":5540,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":102,"record_offset":17428,"parent_record_offset":17264,"ancestor_record_offset":5540,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":102,"record_offset":17232,"parent_record_offset":17172,"ancestor_record_offset":5540,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":102,"record_offset":8700,"parent_record_offset":8672,"ancestor_record_offset":5540,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":5540,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":7060,"parent_record_offset":7036,"ancestor_record_offset":5540,"inlinee":6368,"source_line":717,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":5540,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":7636,"parent_record_offset":7608,"ancestor_record_offset":5540,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":12724,"parent_record_offset":12692,"ancestor_record_offset":5540,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":7664,"parent_record_offset":7636,"ancestor_record_offset":5540,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":102,"record_offset":15472,"parent_record_offset":15436,"ancestor_record_offset":5540,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":5540,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":102,"record_offset":11592,"parent_record_offset":11080,"ancestor_record_offset":5540,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":102,"record_offset":6436,"parent_record_offset":6380,"ancestor_record_offset":5540,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":102,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":5540,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":102,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":5540,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":102,"record_offset":12756,"parent_record_offset":12724,"ancestor_record_offset":5540,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":102,"record_offset":7696,"parent_record_offset":7664,"ancestor_record_offset":5540,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":102,"record_offset":7800,"parent_record_offset":7776,"ancestor_record_offset":5540,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":102,"record_offset":17204,"parent_record_offset":17172,"ancestor_record_offset":5540,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":102,"record_offset":7956,"parent_record_offset":7608,"ancestor_record_offset":5540,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":102,"record_offset":17512,"parent_record_offset":17264,"ancestor_record_offset":5540,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":102,"record_offset":17536,"parent_record_offset":17512,"ancestor_record_offset":5540,"inlinee":8548,"source_line":813,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":102,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":5540,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":7160,"parent_record_offset":7108,"ancestor_record_offset":5540,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":12784,"parent_record_offset":12756,"ancestor_record_offset":5540,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":5540,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":5540,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":16224,"parent_record_offset":16136,"ancestor_record_offset":5540,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":7776,"parent_record_offset":7696,"ancestor_record_offset":5540,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":12620,"parent_record_offset":12512,"ancestor_record_offset":5540,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":12844,"parent_record_offset":12724,"ancestor_record_offset":5540,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":11520,"parent_record_offset":11228,"ancestor_record_offset":5540,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=59
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":6204,"parent_record_offset":5796,"ancestor_record_offset":5540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":102,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":5540,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":5540,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":13968,"parent_record_offset":13936,"ancestor_record_offset":5540,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":6136,"parent_record_offset":6092,"ancestor_record_offset":5540,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":6408,"parent_record_offset":6380,"ancestor_record_offset":5540,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":10520,"parent_record_offset":10492,"ancestor_record_offset":5540,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":10760,"parent_record_offset":10664,"ancestor_record_offset":5540,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":5992,"parent_record_offset":5908,"ancestor_record_offset":5540,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":5540,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":102,"record_offset":7224,"parent_record_offset":7108,"ancestor_record_offset":5540,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":102,"record_offset":7428,"parent_record_offset":7108,"ancestor_record_offset":5540,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":7292,"parent_record_offset":7264,"ancestor_record_offset":5540,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":7264,"parent_record_offset":7224,"ancestor_record_offset":5540,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":102,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":5540,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":5540,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":7852,"parent_record_offset":7800,"ancestor_record_offset":5540,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":6092,"parent_record_offset":6056,"ancestor_record_offset":5540,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":6056,"parent_record_offset":5796,"ancestor_record_offset":5540,"inlinee":9708,"source_line":331,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":19128,"parent_record_offset":19096,"ancestor_record_offset":5540,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":16996,"parent_record_offset":16968,"ancestor_record_offset":5540,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6284,"parent_record_offset":6252,"ancestor_record_offset":5540,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":5936,"parent_record_offset":5908,"ancestor_record_offset":5540,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":10604,"parent_record_offset":10576,"ancestor_record_offset":5540,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6584,"parent_record_offset":6548,"ancestor_record_offset":5540,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":10628,"parent_record_offset":10604,"ancestor_record_offset":5540,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":5540,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":5540,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":5540,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6640,"parent_record_offset":6548,"ancestor_record_offset":5540,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":5540,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":5884,"parent_record_offset":5796,"ancestor_record_offset":5540,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":8260,"parent_record_offset":8020,"ancestor_record_offset":5540,"inlinee":9721,"source_line":1660,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":6228,"parent_record_offset":6204,"ancestor_record_offset":5540,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":16968,"parent_record_offset":16944,"ancestor_record_offset":5540,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":19096,"parent_record_offset":19072,"ancestor_record_offset":5540,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":102,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":5540,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":102,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":5540,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":5540,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":11008,"parent_record_offset":10972,"ancestor_record_offset":5540,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":5540,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":5540,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":9012,"parent_record_offset":8988,"ancestor_record_offset":5540,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":5540,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":5540,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":102,"record_offset":11044,"parent_record_offset":11008,"ancestor_record_offset":5540,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":102,"record_offset":11080,"parent_record_offset":11044,"ancestor_record_offset":5540,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":6172,"parent_record_offset":6056,"ancestor_record_offset":5540,"inlinee":9710,"source_line":440,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::gather_screenshot_requests::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":5540,"inlinee":9717,"source_line":192,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25F5
// 首条位置: {"module":102,"inlinee":9717}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3371
// 首条位置: {"module":223,"inlinee":13169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":223,"record_offset":62660,"parent_record_offset":62608,"ancestor_record_offset":62608,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":223,"record_offset":63172,"parent_record_offset":62608,"ancestor_record_offset":62608,"inlinee":9023,"source_line":1755,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":223,"record_offset":63288,"parent_record_offset":62608,"ancestor_record_offset":62608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":223,"record_offset":62860,"parent_record_offset":62608,"ancestor_record_offset":62608,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":223,"record_offset":62688,"parent_record_offset":62608,"ancestor_record_offset":62608,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":223,"record_offset":63104,"parent_record_offset":63080,"ancestor_record_offset":62608,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":223,"record_offset":63080,"parent_record_offset":63028,"ancestor_record_offset":62608,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":223,"record_offset":63652,"parent_record_offset":63624,"ancestor_record_offset":62608,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":223,"record_offset":63052,"parent_record_offset":63028,"ancestor_record_offset":62608,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":223,"record_offset":63028,"parent_record_offset":63008,"ancestor_record_offset":62608,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":223,"record_offset":64132,"parent_record_offset":64096,"ancestor_record_offset":62608,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":223,"record_offset":64096,"parent_record_offset":64072,"ancestor_record_offset":62608,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":223,"record_offset":63680,"parent_record_offset":63652,"ancestor_record_offset":62608,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":223,"record_offset":63196,"parent_record_offset":63172,"ancestor_record_offset":62608,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":223,"record_offset":63704,"parent_record_offset":63680,"ancestor_record_offset":62608,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":223,"record_offset":62888,"parent_record_offset":62860,"ancestor_record_offset":62608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":223,"record_offset":64164,"parent_record_offset":64132,"ancestor_record_offset":62608,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":223,"record_offset":63224,"parent_record_offset":63196,"ancestor_record_offset":62608,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":223,"record_offset":63252,"parent_record_offset":63196,"ancestor_record_offset":62608,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":223,"record_offset":62796,"parent_record_offset":62712,"ancestor_record_offset":62608,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":223,"record_offset":62820,"parent_record_offset":62796,"ancestor_record_offset":62608,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":223,"record_offset":63976,"parent_record_offset":63944,"ancestor_record_offset":62608,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":223,"record_offset":62740,"parent_record_offset":62712,"ancestor_record_offset":62608,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":223,"record_offset":64000,"parent_record_offset":63976,"ancestor_record_offset":62608,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":223,"record_offset":62764,"parent_record_offset":62740,"ancestor_record_offset":62608,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":223,"record_offset":64192,"parent_record_offset":64132,"ancestor_record_offset":62608,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":223,"record_offset":64224,"parent_record_offset":64096,"ancestor_record_offset":62608,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":223,"record_offset":63920,"parent_record_offset":63896,"ancestor_record_offset":62608,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":223,"record_offset":62712,"parent_record_offset":62688,"ancestor_record_offset":62608,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":223,"record_offset":63944,"parent_record_offset":63920,"ancestor_record_offset":62608,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00223.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::consume_steam_screenshot_request
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 431
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25EB
// 首条位置: {"module":102,"inlinee":9707}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":5856,"parent_record_offset":5796,"ancestor_record_offset":5796,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":6380,"parent_record_offset":5796,"ancestor_record_offset":5796,"inlinee":9718,"source_line":1487,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":6204,"parent_record_offset":5796,"ancestor_record_offset":5796,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":6056,"parent_record_offset":5796,"ancestor_record_offset":5796,"inlinee":9708,"source_line":331,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":5884,"parent_record_offset":5796,"ancestor_record_offset":5796,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":5796,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":6468,"parent_record_offset":6380,"ancestor_record_offset":5796,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":102,"record_offset":6436,"parent_record_offset":6380,"ancestor_record_offset":5796,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":5796,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":6136,"parent_record_offset":6092,"ancestor_record_offset":5796,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":6408,"parent_record_offset":6380,"ancestor_record_offset":5796,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":5992,"parent_record_offset":5908,"ancestor_record_offset":5796,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":5796,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":6092,"parent_record_offset":6056,"ancestor_record_offset":5796,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6284,"parent_record_offset":6252,"ancestor_record_offset":5796,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":5936,"parent_record_offset":5908,"ancestor_record_offset":5796,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6584,"parent_record_offset":6548,"ancestor_record_offset":5796,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":5796,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":5796,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":5796,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":6640,"parent_record_offset":6548,"ancestor_record_offset":5796,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":6228,"parent_record_offset":6204,"ancestor_record_offset":5796,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":102,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":5796,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":102,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":5796,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":6172,"parent_record_offset":6056,"ancestor_record_offset":5796,"inlinee":9710,"source_line":440,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 440
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25EE
// 首条位置: {"module":102,"inlinee":9710}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::add_captured_screenshots_to_steam_library
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25F7
// 首条位置: {"module":102,"inlinee":9719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":8060,"parent_record_offset":8020,"ancestor_record_offset":8020,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":8316,"parent_record_offset":8020,"ancestor_record_offset":8020,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":8088,"parent_record_offset":8020,"ancestor_record_offset":8020,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":8260,"parent_record_offset":8020,"ancestor_record_offset":8020,"inlinee":9721,"source_line":1660,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":8020,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":8196,"parent_record_offset":8112,"ancestor_record_offset":8020,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":8220,"parent_record_offset":8196,"ancestor_record_offset":8020,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":8396,"parent_record_offset":8364,"ancestor_record_offset":8020,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":8020,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":8020,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":8020,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":102,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":8020,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":102,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":8020,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":102,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":8020,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 458
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":73,"record_offset":131576,"rva":952384,"va":5369661504}
// 编译实例 RVAs: 0xe8840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-930357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e8cbcdf1ff","size":5,"soft_fail":false,"target_va":5368732256,"va":5369662608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3087661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e82d1d2f00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369662046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3487904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e8a0383500","size":5,"soft_fail":false,"target_va":5373149840,"va":5369661931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5509156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e824105400","size":5,"soft_fail":false,"target_va":5375171056,"va":5369661895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421446e0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33930394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e89abc0502","size":5,"soft_fail":false,"target_va":5403592416,"va":5369662017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144760
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33930600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e868bd0502","size":5,"soft_fail":false,"target_va":5403592544,"va":5369661939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1994645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e8956f1e00","size":5,"soft_fail":false,"target_va":5371657040,"va":5369662390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33316399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e82f5efc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5369662476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7691960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e8b85e7500","size":5,"soft_fail":false,"target_va":5377353776,"va":5369661811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33306793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e8a938fc01","size":5,"soft_fail":false,"target_va":5402968992,"va":5369662194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t33308333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e8ad3efc01","size":5,"soft_fail":false,"target_va":5402970800,"va":5369662462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7779116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e82cb37600","size":5,"soft_fail":false,"target_va":5377441024,"va":5369661903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] steamworks::screenshots::path_to_absolute_cstring
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6336578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369661504,"raw_hex":"e842b06000","size":5,"soft_fail":false,"target_va":5375998560,"va":5369661977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*(%r9)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5369661504,"raw_hex":"41ff11","size":3,"soft_fail":false,"va":5369661832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":73,"record_offset":132164,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":73,"record_offset":133512,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":73,"record_offset":132192,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":73,"record_offset":132652,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":73,"record_offset":131964,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":73,"record_offset":132680,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":73,"record_offset":132248,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":73,"record_offset":131784,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":73,"record_offset":132220,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":73,"record_offset":132080,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":73,"record_offset":132512,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::screenshots
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-fe4b32a430eda6d8/ef87239/src/lib.rs
//     evidence={"module":73,"record_offset":132716,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":8646,"source_line":442,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] steamworks::screenshots::Screenshots::add_screenshot_to_library
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-fe4b32a430eda6d8/ef87239/src/screenshots.rs
//     evidence={"module":73,"record_offset":132980,"parent_record_offset":131576,"ancestor_record_offset":131576,"inlinee":8648,"source_line":57,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":73,"record_offset":132472,"parent_record_offset":132448,"ancestor_record_offset":131576,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":73,"record_offset":132448,"parent_record_offset":132364,"ancestor_record_offset":131576,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":73,"record_offset":132276,"parent_record_offset":132248,"ancestor_record_offset":131576,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":73,"record_offset":133076,"parent_record_offset":133052,"ancestor_record_offset":131576,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":73,"record_offset":134140,"parent_record_offset":134112,"ancestor_record_offset":131576,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":73,"record_offset":134112,"parent_record_offset":134084,"ancestor_record_offset":131576,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":73,"record_offset":133820,"parent_record_offset":133784,"ancestor_record_offset":131576,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":73,"record_offset":132748,"parent_record_offset":132716,"ancestor_record_offset":131576,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":73,"record_offset":133784,"parent_record_offset":133760,"ancestor_record_offset":131576,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":73,"record_offset":132304,"parent_record_offset":132276,"ancestor_record_offset":131576,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":73,"record_offset":131988,"parent_record_offset":131964,"ancestor_record_offset":131576,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":73,"record_offset":132328,"parent_record_offset":132304,"ancestor_record_offset":131576,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":73,"record_offset":133024,"parent_record_offset":132980,"ancestor_record_offset":131576,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":73,"record_offset":133052,"parent_record_offset":132980,"ancestor_record_offset":131576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":73,"record_offset":133852,"parent_record_offset":133820,"ancestor_record_offset":131576,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":73,"record_offset":132016,"parent_record_offset":131988,"ancestor_record_offset":131576,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":73,"record_offset":132044,"parent_record_offset":131988,"ancestor_record_offset":131576,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":73,"record_offset":131900,"parent_record_offset":131812,"ancestor_record_offset":131576,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":73,"record_offset":131924,"parent_record_offset":131900,"ancestor_record_offset":131576,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":134340,"parent_record_offset":134308,"ancestor_record_offset":131576,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":131844,"parent_record_offset":131812,"ancestor_record_offset":131576,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":132776,"parent_record_offset":132748,"ancestor_record_offset":131576,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":132104,"parent_record_offset":132080,"ancestor_record_offset":131576,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":132800,"parent_record_offset":132776,"ancestor_record_offset":131576,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":134364,"parent_record_offset":134340,"ancestor_record_offset":131576,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":131868,"parent_record_offset":131844,"ancestor_record_offset":131576,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":132128,"parent_record_offset":132104,"ancestor_record_offset":131576,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":133880,"parent_record_offset":133820,"ancestor_record_offset":131576,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":133912,"parent_record_offset":133784,"ancestor_record_offset":131576,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":73,"record_offset":134280,"parent_record_offset":134252,"ancestor_record_offset":131576,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":73,"record_offset":131812,"parent_record_offset":131784,"ancestor_record_offset":131576,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":73,"record_offset":134308,"parent_record_offset":134280,"ancestor_record_offset":131576,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":73,"record_offset":132560,"parent_record_offset":132536,"ancestor_record_offset":131576,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":73,"record_offset":132608,"parent_record_offset":132584,"ancestor_record_offset":131576,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":73,"record_offset":132584,"parent_record_offset":132560,"ancestor_record_offset":131576,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":73,"record_offset":132536,"parent_record_offset":132512,"ancestor_record_offset":131576,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::next_requested_screenshot_stage
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 495
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x261A
// 首条位置: {"module":197,"inlinee":9754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":26232,"parent_record_offset":26012,"ancestor_record_offset":26012,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":197,"record_offset":26384,"parent_record_offset":26012,"ancestor_record_offset":26012,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":26412,"parent_record_offset":26012,"ancestor_record_offset":26012,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":26260,"parent_record_offset":26012,"ancestor_record_offset":26012,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":197,"record_offset":26056,"parent_record_offset":26012,"ancestor_record_offset":26012,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":26012,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":26296,"parent_record_offset":26260,"ancestor_record_offset":26012,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":26168,"parent_record_offset":26080,"ancestor_record_offset":26012,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":197,"record_offset":26192,"parent_record_offset":26168,"ancestor_record_offset":26012,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":26324,"parent_record_offset":26260,"ancestor_record_offset":26012,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":26488,"parent_record_offset":26460,"ancestor_record_offset":26012,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":26112,"parent_record_offset":26080,"ancestor_record_offset":26012,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":26512,"parent_record_offset":26488,"ancestor_record_offset":26012,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":26136,"parent_record_offset":26112,"ancestor_record_offset":26012,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":197,"record_offset":26436,"parent_record_offset":26412,"ancestor_record_offset":26012,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":197,"record_offset":26080,"parent_record_offset":26056,"ancestor_record_offset":26012,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":197,"record_offset":26460,"parent_record_offset":26436,"ancestor_record_offset":26012,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::handle_screenshot_opportunity
// 记录形式: compiled_procedure; instances=1
// 源码行: 511
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":102,"record_offset":21448,"rva":1421728,"va":5370130848}
// 编译实例 RVAs: 0x15b1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=2
//     target_aliases=1889 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1413095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e81970eaff","size":5,"soft_fail":false,"target_va":5368720672,"va":5370133762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a6e50
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t309164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8acb70400","size":5,"soft_fail":false,"target_va":5370441296,"va":5370132127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a8420
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t314098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8f2ca0400","size":5,"soft_fail":false,"target_va":5370446880,"va":5370132777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cc830
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t463093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8f5100700","size":5,"soft_fail":false,"target_va":5370595376,"va":5370132278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cd1a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t465439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e81f1a0700","size":5,"soft_fail":false,"target_va":5370597792,"va":5370132348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14026c110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1117433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8f90c1100","size":5,"soft_fail":false,"target_va":5371248912,"va":5370131474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fb880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1702747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e85bfb1900","size":5,"soft_fail":false,"target_va":5371836544,"va":5370133792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fbb10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1703333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8a5fd1900","size":5,"soft_fail":false,"target_va":5371837200,"va":5370133862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2131201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e801852000","size":5,"soft_fail":false,"target_va":5372267424,"va":5370136218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403779e0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2209262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8eeb52100","size":5,"soft_fail":false,"target_va":5372344800,"va":5370135533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391980
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2316090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e83a572300","size":5,"soft_fail":false,"target_va":5372451200,"va":5370135105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4384503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8f7e64200","size":5,"soft_fail":false,"target_va":5374516480,"va":5370131972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6106532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8a42d5d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5370133383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072edf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6108089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8b9335d00","size":5,"soft_fail":false,"target_va":5376241136,"va":5370133042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072ef10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6108125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8dd335d00","size":5,"soft_fail":false,"target_va":5376241424,"va":5370133294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6109291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e86b385d00","size":5,"soft_fail":false,"target_va":5376242976,"va":5370133680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6150904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8f8da5d00","size":5,"soft_fail":false,"target_va":5376283792,"va":5370132883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407480f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6212668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e83ccc5e00","size":5,"soft_fail":false,"target_va":5376344304,"va":5370131631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140748250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6210720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8a0c45e00","size":5,"soft_fail":false,"target_va":5376344656,"va":5370133931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33461060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e84493fe01","size":5,"soft_fail":false,"target_va":5403593360,"va":5370132295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144de0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33462527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8ff98fe01","size":5,"soft_fail":false,"target_va":5403594208,"va":5370131676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33462602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e84a99fe01","size":5,"soft_fail":false,"target_va":5403594288,"va":5370131681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t32816012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e88cbbf401","size":5,"soft_fail":false,"target_va":5402950480,"va":5370134463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32844207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8af29f501","size":5,"soft_fail":false,"target_va":5402978880,"va":5370134668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_storage_buffer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4077919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e85f393e00","size":5,"soft_fail":false,"target_va":5374211136,"va":5370133212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4081209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e839463e00","size":5,"soft_fail":false,"target_va":5374213616,"va":5370132402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t32834960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e89005f501","size":5,"soft_fail":false,"target_va":5402968992,"va":5370134027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t32836636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e81c0cf501","size":5,"soft_fail":false,"target_va":5402970800,"va":5370134159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6108117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8d5335d00","size":5,"soft_fail":false,"target_va":5376240976,"va":5370132854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6210467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e8a3c35e00","size":5,"soft_fail":false,"target_va":5376341760,"va":5370131288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::half_res
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6209808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e810c15e00","size":5,"soft_fail":false,"target_va":5376341664,"va":5370131851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6209115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e85bbe5e00","size":5,"soft_fail":false,"target_va":5376340512,"va":5370131392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6209293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e80dbf5e00","size":5,"soft_fail":false,"target_va":5376340720,"va":5370131422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6209340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e83cbf5e00","size":5,"soft_fail":false,"target_va":5376340752,"va":5370131407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6087532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370130848,"raw_hex":"e86ce35c00","size":5,"soft_fail":false,"target_va":5376219552,"va":5370132015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":23804,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":21848,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":21880,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::product
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":102,"record_offset":22984,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":6237,"source_line":3606,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":22148,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":23428,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":22204,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":24752,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":21992,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":102,"record_offset":22940,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":102,"record_offset":22896,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":21668,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":24004,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::read
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":102,"record_offset":23744,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":7415,"source_line":210,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":102,"record_offset":23684,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":9083,"source_line":27,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":102,"record_offset":23400,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":8356,"source_line":69,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":102,"record_offset":22240,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":8307,"source_line":61,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::log2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":102,"record_offset":22268,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":6459,"source_line":569,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":102,"record_offset":22324,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":22116,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":9725,"source_line":52,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::progressive_blur_downsample
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":22352,"parent_record_offset":21448,"ancestor_record_offset":21448,"inlinee":9726,"source_line":852,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":102,"record_offset":23032,"parent_record_offset":23008,"ancestor_record_offset":21448,"inlinee":6065,"source_line":262,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":102,"record_offset":23128,"parent_record_offset":23104,"ancestor_record_offset":21448,"inlinee":6073,"source_line":267,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":24028,"parent_record_offset":24004,"ancestor_record_offset":21448,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":102,"record_offset":22624,"parent_record_offset":22352,"ancestor_record_offset":21448,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":102,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":21448,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":102,"record_offset":22592,"parent_record_offset":22528,"ancestor_record_offset":21448,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":102,"record_offset":23056,"parent_record_offset":23032,"ancestor_record_offset":21448,"inlinee":6067,"source_line":40,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":102,"record_offset":22792,"parent_record_offset":22768,"ancestor_record_offset":21448,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":102,"record_offset":22768,"parent_record_offset":22352,"ancestor_record_offset":21448,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":102,"record_offset":23008,"parent_record_offset":22984,"ancestor_record_offset":21448,"inlinee":9731,"source_line":60,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":102,"record_offset":23236,"parent_record_offset":23128,"ancestor_record_offset":21448,"inlinee":9733,"source_line":64,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":102,"record_offset":23080,"parent_record_offset":23056,"ancestor_record_offset":21448,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":102,"record_offset":23152,"parent_record_offset":23128,"ancestor_record_offset":21448,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\try_trait.rs
//     evidence={"module":102,"record_offset":23104,"parent_record_offset":23080,"ancestor_record_offset":21448,"inlinee":6071,"source_line":390,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":22468,"parent_record_offset":22352,"ancestor_record_offset":21448,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":22172,"parent_record_offset":22148,"ancestor_record_offset":21448,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":22852,"parent_record_offset":22352,"ancestor_record_offset":21448,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":23176,"parent_record_offset":23152,"ancestor_record_offset":21448,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":23452,"parent_record_offset":23428,"ancestor_record_offset":21448,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":102,"record_offset":21952,"parent_record_offset":21928,"ancestor_record_offset":21448,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":22080,"parent_record_offset":22056,"ancestor_record_offset":21448,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":22028,"parent_record_offset":21992,"ancestor_record_offset":21448,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":23200,"parent_record_offset":23176,"ancestor_record_offset":21448,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":21784,"parent_record_offset":21696,"ancestor_record_offset":21448,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":21808,"parent_record_offset":21784,"ancestor_record_offset":21448,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":102,"record_offset":21904,"parent_record_offset":21880,"ancestor_record_offset":21448,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":102,"record_offset":21928,"parent_record_offset":21904,"ancestor_record_offset":21448,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":22056,"parent_record_offset":21992,"ancestor_record_offset":21448,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":24316,"parent_record_offset":24284,"ancestor_record_offset":21448,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":21728,"parent_record_offset":21696,"ancestor_record_offset":21448,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":24340,"parent_record_offset":24316,"ancestor_record_offset":21448,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":21752,"parent_record_offset":21728,"ancestor_record_offset":21448,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":102,"record_offset":22724,"parent_record_offset":22352,"ancestor_record_offset":21448,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":102,"record_offset":22680,"parent_record_offset":22352,"ancestor_record_offset":21448,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":24260,"parent_record_offset":24236,"ancestor_record_offset":21448,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":21696,"parent_record_offset":21668,"ancestor_record_offset":21448,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":24284,"parent_record_offset":24260,"ancestor_record_offset":21448,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":102,"record_offset":22648,"parent_record_offset":22624,"ancestor_record_offset":21448,"inlinee":8321,"source_line":49,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":102,"record_offset":22564,"parent_record_offset":22528,"ancestor_record_offset":21448,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":102,"record_offset":22528,"parent_record_offset":22352,"ancestor_record_offset":21448,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":102,"record_offset":22500,"parent_record_offset":22352,"ancestor_record_offset":21448,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::log2f32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\mod.rs
//     evidence={"module":102,"record_offset":22292,"parent_record_offset":22268,"ancestor_record_offset":21448,"inlinee":6461,"source_line":86,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::handle_screenshot_opportunity::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 618
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D3
// 首条位置: {"module":71,"inlinee":8403}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::finish_screenshot_capture
// 记录形式: compiled_procedure; instances=1
// 源码行: 646
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":102,"record_offset":25568,"rva":1428352,"va":5370137472}
// 编译实例 RVAs: 0x15cb80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dfe40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-512143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e8712ff8ff","size":5,"soft_fail":false,"target_va":5369626176,"va":5370138314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32840304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e8701af501","size":5,"soft_fail":false,"target_va":5402978688,"va":5370138379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e88ebaffff","size":5,"soft_fail":false,"target_va":5370120768,"va":5370138541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32842165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e8b521f501","size":5,"soft_fail":false,"target_va":5402980128,"va":5370137958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e842bfffff","size":5,"soft_fail":false,"target_va":5370120960,"va":5370137529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32843052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e82c25f501","size":5,"soft_fail":false,"target_va":5402980656,"va":5370137599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32830813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e85df5f401","size":5,"soft_fail":false,"target_va":5402968992,"va":5370138174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t32832453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e8c5fbf401","size":5,"soft_fail":false,"target_va":5402970800,"va":5370138342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6218785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e821e45e00","size":5,"soft_fail":false,"target_va":5376356912,"va":5370138122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6222666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e84af35e00","size":5,"soft_fail":false,"target_va":5376360560,"va":5370137889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6218639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e88fe35e00","size":5,"soft_fail":false,"target_va":5376356144,"va":5370137500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6178243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e8c3455e00","size":5,"soft_fail":false,"target_va":5376316512,"va":5370138264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2487254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e8d6f32500","size":5,"soft_fail":false,"target_va":5372625168,"va":5370137909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::read_back_screenshot
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370137472,"raw_hex":"e839040000","size":5,"soft_fail":false,"target_va":5370139360,"va":5370138274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-17604","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5370137472,"raw_hex":"e93cbbffff","size":5,"soft_fail":false,"target_va":5370120768,"va":5370138367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":28568,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":28804,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":28860,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":28680,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":26316,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":27536,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":25732,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":26524,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":27920,"parent_record_offset":27892,"ancestor_record_offset":25568,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":27972,"parent_record_offset":27948,"ancestor_record_offset":25568,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":27948,"parent_record_offset":27920,"ancestor_record_offset":25568,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":27860,"parent_record_offset":27832,"ancestor_record_offset":25568,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":27892,"parent_record_offset":27860,"ancestor_record_offset":25568,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":28064,"parent_record_offset":27892,"ancestor_record_offset":25568,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":28176,"parent_record_offset":28152,"ancestor_record_offset":25568,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":25568,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":102,"record_offset":25952,"parent_record_offset":25856,"ancestor_record_offset":25568,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":27756,"parent_record_offset":27728,"ancestor_record_offset":25568,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":28828,"parent_record_offset":28804,"ancestor_record_offset":25568,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":27120,"parent_record_offset":26660,"ancestor_record_offset":25568,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":27804,"parent_record_offset":27632,"ancestor_record_offset":25568,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":26264,"parent_record_offset":26236,"ancestor_record_offset":25568,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":28128,"parent_record_offset":27832,"ancestor_record_offset":25568,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":102,"record_offset":28640,"parent_record_offset":28616,"ancestor_record_offset":25568,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":28768,"parent_record_offset":28744,"ancestor_record_offset":25568,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":26896,"parent_record_offset":26872,"ancestor_record_offset":25568,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":26872,"parent_record_offset":26848,"ancestor_record_offset":25568,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":27080,"parent_record_offset":27048,"ancestor_record_offset":25568,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":28020,"parent_record_offset":27996,"ancestor_record_offset":25568,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":28528,"parent_record_offset":27600,"ancestor_record_offset":25568,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":26452,"parent_record_offset":26344,"ancestor_record_offset":25568,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":26480,"parent_record_offset":26452,"ancestor_record_offset":25568,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":102,"record_offset":25888,"parent_record_offset":25856,"ancestor_record_offset":25568,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":102,"record_offset":28592,"parent_record_offset":28568,"ancestor_record_offset":25568,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":25916,"parent_record_offset":25888,"ancestor_record_offset":25568,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":26848,"parent_record_offset":26824,"ancestor_record_offset":25568,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":102,"record_offset":28616,"parent_record_offset":28592,"ancestor_record_offset":25568,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":28744,"parent_record_offset":28680,"ancestor_record_offset":25568,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":27176,"parent_record_offset":27152,"ancestor_record_offset":25568,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":26824,"parent_record_offset":26716,"ancestor_record_offset":25568,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":102,"record_offset":25856,"parent_record_offset":25828,"ancestor_record_offset":25568,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":102,"record_offset":26556,"parent_record_offset":26524,"ancestor_record_offset":25568,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":102,"record_offset":26688,"parent_record_offset":26660,"ancestor_record_offset":25568,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":102,"record_offset":26660,"parent_record_offset":26628,"ancestor_record_offset":25568,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":26604,"parent_record_offset":26580,"ancestor_record_offset":25568,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":26628,"parent_record_offset":26604,"ancestor_record_offset":25568,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":26580,"parent_record_offset":26556,"ancestor_record_offset":25568,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":27152,"parent_record_offset":26628,"ancestor_record_offset":25568,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":26052,"parent_record_offset":26020,"ancestor_record_offset":25568,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":26212,"parent_record_offset":26052,"ancestor_record_offset":25568,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":26020,"parent_record_offset":25996,"ancestor_record_offset":25568,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":25768,"parent_record_offset":25732,"ancestor_record_offset":25568,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":26236,"parent_record_offset":26212,"ancestor_record_offset":25568,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":25828,"parent_record_offset":25796,"ancestor_record_offset":25568,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":25796,"parent_record_offset":25768,"ancestor_record_offset":25568,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":26716,"parent_record_offset":26688,"ancestor_record_offset":25568,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":26964,"parent_record_offset":26716,"ancestor_record_offset":25568,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":26936,"parent_record_offset":26716,"ancestor_record_offset":25568,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":26768,"parent_record_offset":26716,"ancestor_record_offset":25568,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":28940,"parent_record_offset":28908,"ancestor_record_offset":25568,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":26384,"parent_record_offset":26344,"ancestor_record_offset":25568,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":28964,"parent_record_offset":28940,"ancestor_record_offset":25568,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":26416,"parent_record_offset":26384,"ancestor_record_offset":25568,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":28884,"parent_record_offset":28860,"ancestor_record_offset":25568,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":26344,"parent_record_offset":26316,"ancestor_record_offset":25568,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":28908,"parent_record_offset":28884,"ancestor_record_offset":25568,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":27568,"parent_record_offset":27536,"ancestor_record_offset":25568,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":102,"record_offset":27668,"parent_record_offset":27632,"ancestor_record_offset":25568,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":27832,"parent_record_offset":27632,"ancestor_record_offset":25568,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":28100,"parent_record_offset":27832,"ancestor_record_offset":25568,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":102,"record_offset":27696,"parent_record_offset":27668,"ancestor_record_offset":25568,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":102,"record_offset":27728,"parent_record_offset":27696,"ancestor_record_offset":25568,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":102,"record_offset":27632,"parent_record_offset":27600,"ancestor_record_offset":25568,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":102,"record_offset":27600,"parent_record_offset":27568,"ancestor_record_offset":25568,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::finish_screenshot_capture::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 678
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x214D
// 首条位置: {"module":73,"inlinee":8525}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::hurry_active_screenshots_on_shutdown
// 记录形式: compiled_procedure; instances=1
// 源码行: 681
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":102,"record_offset":29216,"rva":1429440,"va":5370138560}
// 编译实例 RVAs: 0x15cfc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140392ef0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2317689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370138560,"raw_hex":"e8795d2300","size":5,"soft_fail":false,"target_va":5372456688,"va":5370138994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2610724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370138560,"raw_hex":"e824d62700","size":5,"soft_fail":false,"target_va":5372749712,"va":5370138983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] once_cell::imp::OnceCell$LT$T$GT$::initialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32782967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370138560,"raw_hex":"e8773af401","size":5,"soft_fail":false,"target_va":5402922080,"va":5370139108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t32829946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370138560,"raw_hex":"e8faf1f401","size":5,"soft_fail":false,"target_va":5402968992,"va":5370139041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t32831719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370138560,"raw_hex":"e8e7f8f401","size":5,"soft_fail":false,"target_va":5402970800,"va":5370139076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::JoinHandle$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t527936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370138560,"raw_hex":"e8400e0800","size":5,"soft_fail":false,"target_va":5370666880,"va":5370138939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*(%rdx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5370138560,"raw_hex":"ff12","size":2,"soft_fail":false,"va":5370138960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":30716,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":102,"record_offset":30980,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":29936,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":29396,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":29672,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":102,"record_offset":30952,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":30068,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":29968,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":29756,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":30252,"parent_record_offset":29216,"ancestor_record_offset":29216,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":31336,"parent_record_offset":31312,"ancestor_record_offset":29216,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":31312,"parent_record_offset":31228,"ancestor_record_offset":29216,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":31228,"parent_record_offset":31200,"ancestor_record_offset":29216,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":30832,"parent_record_offset":30808,"ancestor_record_offset":29216,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":30808,"parent_record_offset":30716,"ancestor_record_offset":29216,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":30752,"parent_record_offset":30716,"ancestor_record_offset":29216,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":30780,"parent_record_offset":30716,"ancestor_record_offset":29216,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":29484,"parent_record_offset":29460,"ancestor_record_offset":29216,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":29460,"parent_record_offset":29420,"ancestor_record_offset":29216,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":102,"record_offset":29696,"parent_record_offset":29672,"ancestor_record_offset":29216,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":102,"record_offset":29420,"parent_record_offset":29396,"ancestor_record_offset":29216,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":102,"record_offset":31256,"parent_record_offset":31228,"ancestor_record_offset":29216,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":31280,"parent_record_offset":31256,"ancestor_record_offset":29216,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":29532,"parent_record_offset":29484,"ancestor_record_offset":29216,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":31140,"parent_record_offset":30980,"ancestor_record_offset":29216,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":29596,"parent_record_offset":29484,"ancestor_record_offset":29216,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":29720,"parent_record_offset":29696,"ancestor_record_offset":29216,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":31200,"parent_record_offset":31172,"ancestor_record_offset":29216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":102,"record_offset":30912,"parent_record_offset":30888,"ancestor_record_offset":29216,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":31104,"parent_record_offset":31080,"ancestor_record_offset":29216,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":30028,"parent_record_offset":29968,"ancestor_record_offset":29216,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":29872,"parent_record_offset":29784,"ancestor_record_offset":29216,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":29896,"parent_record_offset":29872,"ancestor_record_offset":29216,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":102,"record_offset":30864,"parent_record_offset":30716,"ancestor_record_offset":29216,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":102,"record_offset":30888,"parent_record_offset":30864,"ancestor_record_offset":29216,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":31080,"parent_record_offset":31004,"ancestor_record_offset":29216,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":29216,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":30424,"parent_record_offset":30396,"ancestor_record_offset":29216,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":30156,"parent_record_offset":30124,"ancestor_record_offset":29216,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":29816,"parent_record_offset":29784,"ancestor_record_offset":29216,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":30180,"parent_record_offset":30156,"ancestor_record_offset":29216,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":29840,"parent_record_offset":29816,"ancestor_record_offset":29216,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":30452,"parent_record_offset":30424,"ancestor_record_offset":29216,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":30096,"parent_record_offset":30068,"ancestor_record_offset":29216,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":102,"record_offset":30396,"parent_record_offset":30368,"ancestor_record_offset":29216,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":30280,"parent_record_offset":30252,"ancestor_record_offset":29216,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":30368,"parent_record_offset":30336,"ancestor_record_offset":29216,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":30308,"parent_record_offset":30280,"ancestor_record_offset":29216,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":30336,"parent_record_offset":30308,"ancestor_record_offset":29216,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":29784,"parent_record_offset":29756,"ancestor_record_offset":29216,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":30124,"parent_record_offset":30096,"ancestor_record_offset":29216,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::read_back_screenshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 708
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":102,"record_offset":31896,"rva":1430240,"va":5370139360}
// 编译实例 RVAs: 0x15d2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dfcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-515950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e89220f8ff","size":5,"soft_fail":false,"target_va":5369625776,"va":5370141721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2171597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8cd222100","size":5,"soft_fail":false,"target_va":5372314208,"va":5370142606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fc40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2235716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8441d2200","size":5,"soft_fail":false,"target_va":5372378176,"va":5370142455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387750
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2269433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8f9a02200","size":5,"soft_fail":false,"target_va":5372409680,"va":5370140242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2608516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e884cd2700","size":5,"soft_fail":false,"target_va":5372749696,"va":5370141175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2609474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e842d12700","size":5,"soft_fail":false,"target_va":5372749712,"va":5370140233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6097824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8a00b5d00","size":5,"soft_fail":false,"target_va":5376239920,"va":5370142091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33452040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e80870fe01","size":5,"soft_fail":false,"target_va":5403593360,"va":5370141315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32835853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e80d09f501","size":5,"soft_fail":false,"target_va":5402978144,"va":5370142286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1165142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e856c71100","size":5,"soft_fail":false,"target_va":5371306912,"va":5370141765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7384762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8baae7000","size":5,"soft_fail":false,"target_va":5377527120,"va":5370142353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7222255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8ef336e00","size":5,"soft_fail":false,"target_va":5377364336,"va":5370142076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32836446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e85e0bf501","size":5,"soft_fail":false,"target_va":5402978688,"va":5370142237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32836655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e82f0cf501","size":5,"soft_fail":false,"target_va":5402978880,"va":5370142220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-21097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e897adffff","size":5,"soft_fail":false,"target_va":5370120768,"va":5370141860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::screenshot::ScreenshotSink> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1313937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e86ff3ebff","size":5,"soft_fail":false,"target_va":5368829648,"va":5370143580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32840277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8551af501","size":5,"soft_fail":false,"target_va":5402980128,"va":5370139846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-18477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8d3b7ffff","size":5,"soft_fail":false,"target_va":5370120960,"va":5370139432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32841149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8bd1df501","size":5,"soft_fail":false,"target_va":5402980656,"va":5370139502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7211707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8bb0a6e00","size":5,"soft_fail":false,"target_va":5377353776,"va":5370142064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] once_cell::imp::OnceCell$LT$T$GT$::initialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32780412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e87c30f401","size":5,"soft_fail":false,"target_va":5402922080,"va":5370141663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32827285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e895e7f401","size":5,"soft_fail":false,"target_va":5402968992,"va":5370141702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t32828953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e819eef401","size":5,"soft_fail":false,"target_va":5402970800,"va":5370141842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6216894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8bedc5e00","size":5,"soft_fail":false,"target_va":5376356912,"va":5370140013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t6220778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8eaeb5e00","size":5,"soft_fail":false,"target_va":5376360560,"va":5370139777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6216736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e820dc5e00","size":5,"soft_fail":false,"target_va":5376356144,"va":5370139403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2485366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e876ec2500","size":5,"soft_fail":false,"target_va":5372625168,"va":5370139797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7251546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e85aa66e00","size":5,"soft_fail":false,"target_va":5377393008,"va":5370141457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t541315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e883420800","size":5,"soft_fail":false,"target_va":5370682912,"va":5370141592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::write_image_with_callback_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370139360,"raw_hex":"e8170d0000","size":5,"soft_fail":false,"target_va":5370143824,"va":5370140468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%r8)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5370139360,"raw_hex":"41ff5018","size":4,"soft_fail":false,"va":5370140202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":102,"record_offset":42160,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":102,"record_offset":42188,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":34724,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":34840,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":42244,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":102,"record_offset":42216,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":102,"record_offset":42076,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":33656,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":32068,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":32644,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::call
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":34996,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":9734,"source_line":105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::write_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":35488,"parent_record_offset":31896,"ancestor_record_offset":31896,"inlinee":9735,"source_line":739,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":39016,"parent_record_offset":38992,"ancestor_record_offset":31896,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":39040,"parent_record_offset":39016,"ancestor_record_offset":31896,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":31896,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":38992,"parent_record_offset":38956,"ancestor_record_offset":31896,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":35276,"parent_record_offset":35192,"ancestor_record_offset":31896,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$47::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":35164,"parent_record_offset":34996,"ancestor_record_offset":31896,"inlinee":5521,"source_line":2019,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":35192,"parent_record_offset":35164,"ancestor_record_offset":31896,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":41976,"parent_record_offset":41948,"ancestor_record_offset":31896,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":41228,"parent_record_offset":41196,"ancestor_record_offset":31896,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":38956,"parent_record_offset":38928,"ancestor_record_offset":31896,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":38928,"parent_record_offset":38900,"ancestor_record_offset":31896,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":39112,"parent_record_offset":38928,"ancestor_record_offset":31896,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":41768,"parent_record_offset":41744,"ancestor_record_offset":31896,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38860,"parent_record_offset":38824,"ancestor_record_offset":31896,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38824,"parent_record_offset":38788,"ancestor_record_offset":31896,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":39360,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38788,"parent_record_offset":38752,"ancestor_record_offset":31896,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":40164,"parent_record_offset":40128,"ancestor_record_offset":31896,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":102,"record_offset":40128,"parent_record_offset":40092,"ancestor_record_offset":31896,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":102,"record_offset":40092,"parent_record_offset":40056,"ancestor_record_offset":31896,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":38520,"parent_record_offset":38452,"ancestor_record_offset":31896,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":41196,"parent_record_offset":41152,"ancestor_record_offset":31896,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":41152,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":38900,"parent_record_offset":38860,"ancestor_record_offset":31896,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":38752,"parent_record_offset":38712,"ancestor_record_offset":31896,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":102,"record_offset":35220,"parent_record_offset":35192,"ancestor_record_offset":31896,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":34076,"parent_record_offset":34048,"ancestor_record_offset":31896,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":34128,"parent_record_offset":34104,"ancestor_record_offset":31896,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":34104,"parent_record_offset":34076,"ancestor_record_offset":31896,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":34016,"parent_record_offset":33984,"ancestor_record_offset":31896,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":34048,"parent_record_offset":34016,"ancestor_record_offset":31896,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":34220,"parent_record_offset":34048,"ancestor_record_offset":31896,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":34332,"parent_record_offset":34308,"ancestor_record_offset":31896,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":102,"record_offset":39224,"parent_record_offset":38712,"ancestor_record_offset":31896,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":102,"record_offset":40056,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":102,"record_offset":39176,"parent_record_offset":39152,"ancestor_record_offset":31896,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":34152,"parent_record_offset":34128,"ancestor_record_offset":31896,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":35244,"parent_record_offset":35220,"ancestor_record_offset":31896,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":102,"record_offset":32280,"parent_record_offset":32184,"ancestor_record_offset":31896,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":33908,"parent_record_offset":33880,"ancestor_record_offset":31896,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":33240,"parent_record_offset":32780,"ancestor_record_offset":31896,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":33956,"parent_record_offset":33776,"ancestor_record_offset":31896,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":35044,"parent_record_offset":34996,"ancestor_record_offset":31896,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":35128,"parent_record_offset":34996,"ancestor_record_offset":31896,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":32592,"parent_record_offset":32564,"ancestor_record_offset":31896,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":39152,"parent_record_offset":38860,"ancestor_record_offset":31896,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":34284,"parent_record_offset":33984,"ancestor_record_offset":31896,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":33016,"parent_record_offset":32992,"ancestor_record_offset":31896,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":32992,"parent_record_offset":32968,"ancestor_record_offset":31896,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":33200,"parent_record_offset":33168,"ancestor_record_offset":31896,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":34776,"parent_record_offset":34748,"ancestor_record_offset":31896,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":39068,"parent_record_offset":39040,"ancestor_record_offset":31896,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":34176,"parent_record_offset":34152,"ancestor_record_offset":31896,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":34684,"parent_record_offset":33736,"ancestor_record_offset":31896,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":41088,"parent_record_offset":41004,"ancestor_record_offset":31896,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":41112,"parent_record_offset":41088,"ancestor_record_offset":31896,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":102,"record_offset":32216,"parent_record_offset":32184,"ancestor_record_offset":31896,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":32244,"parent_record_offset":32216,"ancestor_record_offset":31896,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":32968,"parent_record_offset":32944,"ancestor_record_offset":31896,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":33296,"parent_record_offset":33272,"ancestor_record_offset":31896,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":32944,"parent_record_offset":32836,"ancestor_record_offset":31896,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":102,"record_offset":32184,"parent_record_offset":32156,"ancestor_record_offset":31896,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":102,"record_offset":32676,"parent_record_offset":32644,"ancestor_record_offset":31896,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":102,"record_offset":32808,"parent_record_offset":32780,"ancestor_record_offset":31896,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":102,"record_offset":32780,"parent_record_offset":32748,"ancestor_record_offset":31896,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":32724,"parent_record_offset":32700,"ancestor_record_offset":31896,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":32748,"parent_record_offset":32724,"ancestor_record_offset":31896,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":32700,"parent_record_offset":32676,"ancestor_record_offset":31896,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":33272,"parent_record_offset":32748,"ancestor_record_offset":31896,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":32380,"parent_record_offset":32348,"ancestor_record_offset":31896,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":32540,"parent_record_offset":32380,"ancestor_record_offset":31896,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":32348,"parent_record_offset":32324,"ancestor_record_offset":31896,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":32100,"parent_record_offset":32068,"ancestor_record_offset":31896,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":32564,"parent_record_offset":32540,"ancestor_record_offset":31896,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":32156,"parent_record_offset":32124,"ancestor_record_offset":31896,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":32124,"parent_record_offset":32100,"ancestor_record_offset":31896,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":32836,"parent_record_offset":32808,"ancestor_record_offset":31896,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":33084,"parent_record_offset":32836,"ancestor_record_offset":31896,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":33056,"parent_record_offset":32836,"ancestor_record_offset":31896,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":32888,"parent_record_offset":32836,"ancestor_record_offset":31896,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":40900,"parent_record_offset":40876,"ancestor_record_offset":31896,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":41372,"parent_record_offset":41340,"ancestor_record_offset":31896,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":41032,"parent_record_offset":41004,"ancestor_record_offset":31896,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":38544,"parent_record_offset":38520,"ancestor_record_offset":31896,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":42100,"parent_record_offset":42076,"ancestor_record_offset":31896,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":38568,"parent_record_offset":38544,"ancestor_record_offset":31896,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":41396,"parent_record_offset":41372,"ancestor_record_offset":31896,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":41056,"parent_record_offset":41032,"ancestor_record_offset":31896,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":40924,"parent_record_offset":40900,"ancestor_record_offset":31896,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":40980,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":41316,"parent_record_offset":41292,"ancestor_record_offset":31896,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":102,"record_offset":40876,"parent_record_offset":40852,"ancestor_record_offset":31896,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40776,"parent_record_offset":40752,"ancestor_record_offset":31896,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40852,"parent_record_offset":40824,"ancestor_record_offset":31896,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40800,"parent_record_offset":40776,"ancestor_record_offset":31896,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40824,"parent_record_offset":40800,"ancestor_record_offset":31896,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40752,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":41004,"parent_record_offset":40980,"ancestor_record_offset":31896,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":41340,"parent_record_offset":41316,"ancestor_record_offset":31896,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":37384,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":33696,"parent_record_offset":33656,"ancestor_record_offset":31896,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":35668,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":102,"record_offset":33820,"parent_record_offset":33776,"ancestor_record_offset":31896,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":33984,"parent_record_offset":33776,"ancestor_record_offset":31896,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":34256,"parent_record_offset":33984,"ancestor_record_offset":31896,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":36372,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":38640,"parent_record_offset":38604,"ancestor_record_offset":31896,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":38604,"parent_record_offset":38452,"ancestor_record_offset":31896,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":102,"record_offset":38676,"parent_record_offset":38640,"ancestor_record_offset":31896,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":102,"record_offset":33848,"parent_record_offset":33820,"ancestor_record_offset":31896,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":102,"record_offset":33880,"parent_record_offset":33848,"ancestor_record_offset":31896,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":102,"record_offset":38712,"parent_record_offset":38676,"ancestor_record_offset":31896,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":102,"record_offset":40028,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":9739,"source_line":289,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":102,"record_offset":33776,"parent_record_offset":33736,"ancestor_record_offset":31896,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":102,"record_offset":33736,"parent_record_offset":33696,"ancestor_record_offset":31896,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::impl$10::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":38452,"parent_record_offset":35488,"ancestor_record_offset":31896,"inlinee":9737,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 739
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2607
// 首条位置: {"module":102,"inlinee":9735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":39360,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":41152,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":102,"record_offset":40056,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":36256,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":41292,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":40708,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":40980,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40752,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":37384,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":35668,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":36372,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":102,"record_offset":40028,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":9739,"source_line":289,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::impl$10::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":38452,"parent_record_offset":35488,"ancestor_record_offset":35488,"inlinee":9737,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":39016,"parent_record_offset":38992,"ancestor_record_offset":35488,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":39040,"parent_record_offset":39016,"ancestor_record_offset":35488,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":41816,"parent_record_offset":41792,"ancestor_record_offset":35488,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":38992,"parent_record_offset":38956,"ancestor_record_offset":35488,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":41792,"parent_record_offset":41768,"ancestor_record_offset":35488,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":41976,"parent_record_offset":41948,"ancestor_record_offset":35488,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":41228,"parent_record_offset":41196,"ancestor_record_offset":35488,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":38956,"parent_record_offset":38928,"ancestor_record_offset":35488,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":38928,"parent_record_offset":38900,"ancestor_record_offset":35488,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":39112,"parent_record_offset":38928,"ancestor_record_offset":35488,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":41768,"parent_record_offset":41744,"ancestor_record_offset":35488,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38860,"parent_record_offset":38824,"ancestor_record_offset":35488,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38824,"parent_record_offset":38788,"ancestor_record_offset":35488,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":40212,"parent_record_offset":40164,"ancestor_record_offset":35488,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":38788,"parent_record_offset":38752,"ancestor_record_offset":35488,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":102,"record_offset":40164,"parent_record_offset":40128,"ancestor_record_offset":35488,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":102,"record_offset":40128,"parent_record_offset":40092,"ancestor_record_offset":35488,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":102,"record_offset":40092,"parent_record_offset":40056,"ancestor_record_offset":35488,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":38520,"parent_record_offset":38452,"ancestor_record_offset":35488,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":41196,"parent_record_offset":41152,"ancestor_record_offset":35488,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":38900,"parent_record_offset":38860,"ancestor_record_offset":35488,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":38752,"parent_record_offset":38712,"ancestor_record_offset":35488,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":37804,"parent_record_offset":37776,"ancestor_record_offset":35488,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":35488,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":37832,"parent_record_offset":37804,"ancestor_record_offset":35488,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":37744,"parent_record_offset":37712,"ancestor_record_offset":35488,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":37776,"parent_record_offset":37744,"ancestor_record_offset":35488,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":37948,"parent_record_offset":37776,"ancestor_record_offset":35488,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":102,"record_offset":38060,"parent_record_offset":38036,"ancestor_record_offset":35488,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":102,"record_offset":39224,"parent_record_offset":38712,"ancestor_record_offset":35488,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":102,"record_offset":39176,"parent_record_offset":39152,"ancestor_record_offset":35488,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":36280,"parent_record_offset":36256,"ancestor_record_offset":35488,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":102,"record_offset":35892,"parent_record_offset":35796,"ancestor_record_offset":35488,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":37636,"parent_record_offset":37608,"ancestor_record_offset":35488,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":36968,"parent_record_offset":36508,"ancestor_record_offset":35488,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":37684,"parent_record_offset":37504,"ancestor_record_offset":35488,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":36204,"parent_record_offset":36176,"ancestor_record_offset":35488,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":102,"record_offset":39152,"parent_record_offset":38860,"ancestor_record_offset":35488,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":38012,"parent_record_offset":37712,"ancestor_record_offset":35488,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":36744,"parent_record_offset":36720,"ancestor_record_offset":35488,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":36720,"parent_record_offset":36696,"ancestor_record_offset":35488,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":102,"record_offset":36928,"parent_record_offset":36896,"ancestor_record_offset":35488,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":36308,"parent_record_offset":36280,"ancestor_record_offset":35488,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":39068,"parent_record_offset":39040,"ancestor_record_offset":35488,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":36336,"parent_record_offset":36280,"ancestor_record_offset":35488,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":38412,"parent_record_offset":37464,"ancestor_record_offset":35488,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":41088,"parent_record_offset":41004,"ancestor_record_offset":35488,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":41112,"parent_record_offset":41088,"ancestor_record_offset":35488,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":102,"record_offset":35828,"parent_record_offset":35796,"ancestor_record_offset":35488,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":35856,"parent_record_offset":35828,"ancestor_record_offset":35488,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":36696,"parent_record_offset":36672,"ancestor_record_offset":35488,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":37024,"parent_record_offset":37000,"ancestor_record_offset":35488,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":102,"record_offset":36672,"parent_record_offset":36564,"ancestor_record_offset":35488,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":102,"record_offset":35796,"parent_record_offset":35768,"ancestor_record_offset":35488,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":102,"record_offset":36404,"parent_record_offset":36372,"ancestor_record_offset":35488,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":102,"record_offset":36536,"parent_record_offset":36508,"ancestor_record_offset":35488,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":102,"record_offset":36508,"parent_record_offset":36476,"ancestor_record_offset":35488,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":36452,"parent_record_offset":36428,"ancestor_record_offset":35488,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":36476,"parent_record_offset":36452,"ancestor_record_offset":35488,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":36428,"parent_record_offset":36404,"ancestor_record_offset":35488,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":102,"record_offset":37000,"parent_record_offset":36476,"ancestor_record_offset":35488,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":35992,"parent_record_offset":35960,"ancestor_record_offset":35488,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":36152,"parent_record_offset":35992,"ancestor_record_offset":35488,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":35960,"parent_record_offset":35936,"ancestor_record_offset":35488,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":35704,"parent_record_offset":35668,"ancestor_record_offset":35488,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":36176,"parent_record_offset":36152,"ancestor_record_offset":35488,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":35768,"parent_record_offset":35732,"ancestor_record_offset":35488,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":102,"record_offset":35732,"parent_record_offset":35704,"ancestor_record_offset":35488,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":36564,"parent_record_offset":36536,"ancestor_record_offset":35488,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":36812,"parent_record_offset":36564,"ancestor_record_offset":35488,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":36784,"parent_record_offset":36564,"ancestor_record_offset":35488,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":102,"record_offset":36616,"parent_record_offset":36564,"ancestor_record_offset":35488,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":40900,"parent_record_offset":40876,"ancestor_record_offset":35488,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":41372,"parent_record_offset":41340,"ancestor_record_offset":35488,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":41032,"parent_record_offset":41004,"ancestor_record_offset":35488,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":38544,"parent_record_offset":38520,"ancestor_record_offset":35488,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":38568,"parent_record_offset":38544,"ancestor_record_offset":35488,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":41396,"parent_record_offset":41372,"ancestor_record_offset":35488,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":41056,"parent_record_offset":41032,"ancestor_record_offset":35488,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":40924,"parent_record_offset":40900,"ancestor_record_offset":35488,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":41316,"parent_record_offset":41292,"ancestor_record_offset":35488,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":102,"record_offset":40876,"parent_record_offset":40852,"ancestor_record_offset":35488,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40776,"parent_record_offset":40752,"ancestor_record_offset":35488,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40852,"parent_record_offset":40824,"ancestor_record_offset":35488,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40800,"parent_record_offset":40776,"ancestor_record_offset":35488,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":102,"record_offset":40824,"parent_record_offset":40800,"ancestor_record_offset":35488,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":41004,"parent_record_offset":40980,"ancestor_record_offset":35488,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":41340,"parent_record_offset":41316,"ancestor_record_offset":35488,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":37424,"parent_record_offset":37384,"ancestor_record_offset":35488,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":102,"record_offset":37548,"parent_record_offset":37504,"ancestor_record_offset":35488,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":37712,"parent_record_offset":37504,"ancestor_record_offset":35488,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":102,"record_offset":37984,"parent_record_offset":37712,"ancestor_record_offset":35488,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":38640,"parent_record_offset":38604,"ancestor_record_offset":35488,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":38604,"parent_record_offset":38452,"ancestor_record_offset":35488,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":102,"record_offset":38676,"parent_record_offset":38640,"ancestor_record_offset":35488,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":102,"record_offset":37576,"parent_record_offset":37548,"ancestor_record_offset":35488,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":102,"record_offset":37608,"parent_record_offset":37576,"ancestor_record_offset":35488,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":102,"record_offset":38712,"parent_record_offset":38676,"ancestor_record_offset":35488,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":102,"record_offset":37504,"parent_record_offset":37464,"ancestor_record_offset":35488,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":102,"record_offset":37464,"parent_record_offset":37424,"ancestor_record_offset":35488,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 754
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1589
// 首条位置: {"module":14,"inlinee":5513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":14,"record_offset":792,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":14,"record_offset":820,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":876,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":14,"record_offset":848,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":14,"record_offset":708,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":660,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":14,"record_offset":1336,"parent_record_offset":1280,"ancestor_record_offset":660,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":1308,"parent_record_offset":1280,"ancestor_record_offset":660,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":14,"record_offset":1280,"parent_record_offset":1252,"ancestor_record_offset":660,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":992,"parent_record_offset":956,"ancestor_record_offset":660,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":956,"parent_record_offset":928,"ancestor_record_offset":660,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":14,"record_offset":1024,"parent_record_offset":992,"ancestor_record_offset":660,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":660,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":660,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":1052,"parent_record_offset":992,"ancestor_record_offset":660,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":1084,"parent_record_offset":956,"ancestor_record_offset":660,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 764
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2148
// 首条位置: {"module":73,"inlinee":8520}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::thread::JoinHandle::is_finished
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":73,"record_offset":8684,"parent_record_offset":8664,"ancestor_record_offset":8664,"inlinee":8522,"source_line":1672,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":73,"record_offset":8732,"parent_record_offset":8708,"ancestor_record_offset":8664,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::strong_count
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":73,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":8664,"inlinee":8523,"source_line":1703,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":73,"record_offset":8764,"parent_record_offset":8732,"ancestor_record_offset":8664,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":73,"record_offset":8792,"parent_record_offset":8732,"ancestor_record_offset":8664,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_with_callback_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 778
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":102,"record_offset":42584,"rva":1434704,"va":5370143824}
// 编译实例 RVAs: 0x15e450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$image..buffer_..ImageBuffer$LT$FromType$C$Container$GT$$u20$as$u20$image..buffer_..ConvertBuffer$LT$image..buffer_..ImageBuffer$LT$ToType$C$alloc..vec..Vec$LT$$LT$ToType$u20$as$u20$image..traits..Pixel$GT$..Subpixel$GT$$GT$$GT$$GT$::convert
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2087950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e80edc1f00","size":5,"soft_fail":false,"target_va":5372231952,"va":5370143997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$image..codecs..jpeg..encoder..JpegEncoder$LT$W$GT$$u20$as$u20$image..image..ImageEncoder$GT$::write_image
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-161524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e80c89fdff","size":5,"soft_fail":false,"target_va":5369982640,"va":5370144159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140024cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1287404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8145becff","size":5,"soft_fail":false,"target_va":5368859888,"va":5370147287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387750
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2264797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8dd8e2200","size":5,"soft_fail":false,"target_va":5372409680,"va":5370144878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038d0a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2286594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e802e42200","size":5,"soft_fail":false,"target_va":5372432544,"va":5370145945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2604967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8a7bf2700","size":5,"soft_fail":false,"target_va":5372749696,"va":5370144724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2605415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e867c12700","size":5,"soft_fail":false,"target_va":5372749712,"va":5370144292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3004725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e835d92d00","size":5,"soft_fail":false,"target_va":5373149840,"va":5370145110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=4
//     target_aliases=28 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5026126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e84eb14c00","size":5,"soft_fail":false,"target_va":5375171056,"va":5370144925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32756270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e82ed2f301","size":5,"soft_fail":false,"target_va":5402900112,"va":5370143837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33449272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e83865fe01","size":5,"soft_fail":false,"target_va":5403593360,"va":5370144083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32831987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8f3f9f401","size":5,"soft_fail":false,"target_va":5402978144,"va":5370146152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7385195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e86bb07000","size":5,"soft_fail":false,"target_va":5377529888,"va":5370144688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32773039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8af13f401","size":5,"soft_fail":false,"target_va":5402917504,"va":5370144460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t32804902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e82690f401","size":5,"soft_fail":false,"target_va":5402949904,"va":5370144997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7217298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e892206e00","size":5,"soft_fail":false,"target_va":5377364336,"va":5370147033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32832706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8c2fcf401","size":5,"soft_fail":false,"target_va":5402978880,"va":5370146169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32832945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8b1fdf401","size":5,"soft_fail":false,"target_va":5402979152,"va":5370146202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32834049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e80102f501","size":5,"soft_fail":false,"target_va":5402980304,"va":5370146250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::codecs::jpeg::encoder::JpegEncoder$LT$W$GT$::new_with_quality
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-166143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e80177fdff","size":5,"soft_fail":false,"target_va":5369977920,"va":5370144058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::free_functions::save_buffer_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5910276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8042f5a00","size":5,"soft_fail":false,"target_va":5376055456,"va":5370145175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32824734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e89eddf401","size":5,"soft_fail":false,"target_va":5402968992,"va":5370144253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32826449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e851e4f401","size":5,"soft_fail":false,"target_va":5402970800,"va":5370144346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7263770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e81ad66e00","size":5,"soft_fail":false,"target_va":5377408752,"va":5370144977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7262313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e869d06e00","size":5,"soft_fail":false,"target_va":5377408016,"va":5370145698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7294627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e8a34e6f00","size":5,"soft_fail":false,"target_va":5377439664,"va":5370145032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7291999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370143824,"raw_hex":"e85f446f00","size":5,"soft_fail":false,"target_va":5377436944,"va":5370144940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*24(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5370143824,"raw_hex":"ff5718","size":3,"soft_fail":false,"va":5370144832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":102,"record_offset":48096,"parent_record_offset":42584,"ancestor_record_offset":42584,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":102,"record_offset":48180,"parent_record_offset":42584,"ancestor_record_offset":42584,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":49664,"parent_record_offset":42584,"ancestor_record_offset":42584,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::call
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":48680,"parent_record_offset":42584,"ancestor_record_offset":42584,"inlinee":9734,"source_line":105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::write_image_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":42740,"parent_record_offset":42584,"ancestor_record_offset":42584,"inlinee":9740,"source_line":804,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":46560,"parent_record_offset":46536,"ancestor_record_offset":42584,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":46584,"parent_record_offset":46560,"ancestor_record_offset":42584,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":43676,"parent_record_offset":43652,"ancestor_record_offset":42584,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":48288,"parent_record_offset":48252,"ancestor_record_offset":42584,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":46536,"parent_record_offset":46508,"ancestor_record_offset":42584,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":43652,"parent_record_offset":43596,"ancestor_record_offset":42584,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":48252,"parent_record_offset":48216,"ancestor_record_offset":42584,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$47::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":48852,"parent_record_offset":48680,"ancestor_record_offset":42584,"inlinee":5521,"source_line":2019,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$71::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":48216,"parent_record_offset":48180,"ancestor_record_offset":42584,"inlinee":5245,"source_line":2332,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":102,"record_offset":48880,"parent_record_offset":48852,"ancestor_record_offset":42584,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":102,"record_offset":48144,"parent_record_offset":48120,"ancestor_record_offset":42584,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":43624,"parent_record_offset":43596,"ancestor_record_offset":42584,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":46508,"parent_record_offset":46480,"ancestor_record_offset":42584,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":46480,"parent_record_offset":46452,"ancestor_record_offset":42584,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":46656,"parent_record_offset":46480,"ancestor_record_offset":42584,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":43596,"parent_record_offset":43568,"ancestor_record_offset":42584,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":43320,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":43228,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":46424,"parent_record_offset":46392,"ancestor_record_offset":42584,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":46452,"parent_record_offset":46424,"ancestor_record_offset":42584,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":102,"record_offset":48908,"parent_record_offset":48880,"ancestor_record_offset":42584,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":102,"record_offset":44632,"parent_record_offset":44604,"ancestor_record_offset":42584,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":102,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":42584,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":102,"record_offset":44152,"parent_record_offset":44128,"ancestor_record_offset":42584,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":48748,"parent_record_offset":48724,"ancestor_record_offset":42584,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":102,"record_offset":48932,"parent_record_offset":48908,"ancestor_record_offset":42584,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":43036,"parent_record_offset":43016,"ancestor_record_offset":42584,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":43060,"parent_record_offset":43036,"ancestor_record_offset":42584,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":42992,"parent_record_offset":42968,"ancestor_record_offset":42584,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":48120,"parent_record_offset":48096,"ancestor_record_offset":42584,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":44104,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":7550,"source_line":1231,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":48724,"parent_record_offset":48680,"ancestor_record_offset":42584,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":45088,"parent_record_offset":44788,"ancestor_record_offset":42584,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":44580,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":44604,"parent_record_offset":44580,"ancestor_record_offset":42584,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":43520,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":46612,"parent_record_offset":46584,"ancestor_record_offset":42584,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":48772,"parent_record_offset":48748,"ancestor_record_offset":42584,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":43456,"parent_record_offset":43372,"ancestor_record_offset":42584,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":43480,"parent_record_offset":43456,"ancestor_record_offset":42584,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":43284,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":44412,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":102,"record_offset":44680,"parent_record_offset":44656,"ancestor_record_offset":42584,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":102,"record_offset":44704,"parent_record_offset":44680,"ancestor_record_offset":42584,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":45124,"parent_record_offset":44788,"ancestor_record_offset":42584,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":45180,"parent_record_offset":45152,"ancestor_record_offset":42584,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":45152,"parent_record_offset":45124,"ancestor_record_offset":42584,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":43804,"parent_record_offset":43772,"ancestor_record_offset":42584,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":43400,"parent_record_offset":43372,"ancestor_record_offset":42584,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":43828,"parent_record_offset":43804,"ancestor_record_offset":42584,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":43424,"parent_record_offset":43400,"ancestor_record_offset":42584,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":42944,"parent_record_offset":42920,"ancestor_record_offset":42584,"inlinee":7582,"source_line":802,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":42920,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":7581,"source_line":674,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":42968,"parent_record_offset":42944,"ancestor_record_offset":42584,"inlinee":7583,"source_line":807,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":44788,"parent_record_offset":44752,"ancestor_record_offset":42584,"inlinee":9746,"source_line":713,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::save
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":44752,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":9745,"source_line":996,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":43016,"parent_record_offset":42992,"ancestor_record_offset":42584,"inlinee":7585,"source_line":809,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":43128,"parent_record_offset":43104,"ancestor_record_offset":42584,"inlinee":7586,"source_line":810,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":43256,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":9742,"source_line":1110,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::save_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":102,"record_offset":45228,"parent_record_offset":44752,"ancestor_record_offset":42584,"inlinee":7594,"source_line":1111,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":43348,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":43748,"parent_record_offset":43724,"ancestor_record_offset":42584,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":43372,"parent_record_offset":43348,"ancestor_record_offset":42584,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":43772,"parent_record_offset":43748,"ancestor_record_offset":42584,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$40::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":44656,"parent_record_offset":44632,"ancestor_record_offset":42584,"inlinee":8925,"source_line":1348,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":44236,"parent_record_offset":44212,"ancestor_record_offset":42584,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":44284,"parent_record_offset":44260,"ancestor_record_offset":42584,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":44260,"parent_record_offset":44236,"ancestor_record_offset":42584,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":44380,"parent_record_offset":44328,"ancestor_record_offset":42584,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":44352,"parent_record_offset":44328,"ancestor_record_offset":42584,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":45924,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":5265,"source_line":403,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":45948,"parent_record_offset":45924,"ancestor_record_offset":42584,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":46268,"parent_record_offset":45924,"ancestor_record_offset":42584,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":44328,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufwriter::BufWriter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":102,"record_offset":46360,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":9747,"source_line":95,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufwriter::BufWriter::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":102,"record_offset":46392,"parent_record_offset":46360,"ancestor_record_offset":42584,"inlinee":9748,"source_line":113,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":102,"record_offset":46792,"parent_record_offset":46768,"ancestor_record_offset":42584,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":44212,"parent_record_offset":44188,"ancestor_record_offset":42584,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":44188,"parent_record_offset":42740,"ancestor_record_offset":42584,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":42584,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":102,"record_offset":46012,"parent_record_offset":45948,"ancestor_record_offset":42584,"inlinee":5270,"source_line":218,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":102,"record_offset":46040,"parent_record_offset":45948,"ancestor_record_offset":42584,"inlinee":5271,"source_line":215,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":102,"record_offset":45984,"parent_record_offset":45948,"ancestor_record_offset":42584,"inlinee":5269,"source_line":209,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::write_image_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":44128,"parent_record_offset":44104,"ancestor_record_offset":42584,"inlinee":9744,"source_line":810,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 804
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x260C
// 首条位置: {"module":102,"inlinee":9740}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":102,"record_offset":43320,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":43228,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":44104,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":7550,"source_line":1231,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":44580,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":43520,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":43284,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":44412,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":42920,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":7581,"source_line":674,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::save
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":44752,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":9745,"source_line":996,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":43256,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":9742,"source_line":1110,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":43348,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":45924,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":5265,"source_line":403,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":44328,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufwriter::BufWriter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":102,"record_offset":46360,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":9747,"source_line":95,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":44188,"parent_record_offset":42740,"ancestor_record_offset":42740,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":46560,"parent_record_offset":46536,"ancestor_record_offset":42740,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":46584,"parent_record_offset":46560,"ancestor_record_offset":42740,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":43676,"parent_record_offset":43652,"ancestor_record_offset":42740,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":46536,"parent_record_offset":46508,"ancestor_record_offset":42740,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":102,"record_offset":43652,"parent_record_offset":43596,"ancestor_record_offset":42740,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":43624,"parent_record_offset":43596,"ancestor_record_offset":42740,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":46508,"parent_record_offset":46480,"ancestor_record_offset":42740,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":46480,"parent_record_offset":46452,"ancestor_record_offset":42740,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":46656,"parent_record_offset":46480,"ancestor_record_offset":42740,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":102,"record_offset":43596,"parent_record_offset":43568,"ancestor_record_offset":42740,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":46424,"parent_record_offset":46392,"ancestor_record_offset":42740,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":102,"record_offset":46452,"parent_record_offset":46424,"ancestor_record_offset":42740,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":102,"record_offset":44632,"parent_record_offset":44604,"ancestor_record_offset":42740,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":102,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":42740,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":102,"record_offset":44152,"parent_record_offset":44128,"ancestor_record_offset":42740,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":43036,"parent_record_offset":43016,"ancestor_record_offset":42740,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":102,"record_offset":43060,"parent_record_offset":43036,"ancestor_record_offset":42740,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":42992,"parent_record_offset":42968,"ancestor_record_offset":42740,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":45088,"parent_record_offset":44788,"ancestor_record_offset":42740,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":44604,"parent_record_offset":44580,"ancestor_record_offset":42740,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":43544,"parent_record_offset":43520,"ancestor_record_offset":42740,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":102,"record_offset":46612,"parent_record_offset":46584,"ancestor_record_offset":42740,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":43456,"parent_record_offset":43372,"ancestor_record_offset":42740,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":102,"record_offset":43480,"parent_record_offset":43456,"ancestor_record_offset":42740,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":102,"record_offset":44680,"parent_record_offset":44656,"ancestor_record_offset":42740,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":102,"record_offset":44704,"parent_record_offset":44680,"ancestor_record_offset":42740,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":45124,"parent_record_offset":44788,"ancestor_record_offset":42740,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":45180,"parent_record_offset":45152,"ancestor_record_offset":42740,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":102,"record_offset":45152,"parent_record_offset":45124,"ancestor_record_offset":42740,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":43804,"parent_record_offset":43772,"ancestor_record_offset":42740,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":43400,"parent_record_offset":43372,"ancestor_record_offset":42740,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":43828,"parent_record_offset":43804,"ancestor_record_offset":42740,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":102,"record_offset":43424,"parent_record_offset":43400,"ancestor_record_offset":42740,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":42944,"parent_record_offset":42920,"ancestor_record_offset":42740,"inlinee":7582,"source_line":802,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":42968,"parent_record_offset":42944,"ancestor_record_offset":42740,"inlinee":7583,"source_line":807,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":44788,"parent_record_offset":44752,"ancestor_record_offset":42740,"inlinee":9746,"source_line":713,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":43016,"parent_record_offset":42992,"ancestor_record_offset":42740,"inlinee":7585,"source_line":809,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":102,"record_offset":43128,"parent_record_offset":43104,"ancestor_record_offset":42740,"inlinee":7586,"source_line":810,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::save_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":102,"record_offset":45228,"parent_record_offset":44752,"ancestor_record_offset":42740,"inlinee":7594,"source_line":1111,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":102,"record_offset":43748,"parent_record_offset":43724,"ancestor_record_offset":42740,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":43372,"parent_record_offset":43348,"ancestor_record_offset":42740,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":102,"record_offset":43772,"parent_record_offset":43748,"ancestor_record_offset":42740,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$40::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":44656,"parent_record_offset":44632,"ancestor_record_offset":42740,"inlinee":8925,"source_line":1348,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":44236,"parent_record_offset":44212,"ancestor_record_offset":42740,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":44284,"parent_record_offset":44260,"ancestor_record_offset":42740,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":102,"record_offset":44260,"parent_record_offset":44236,"ancestor_record_offset":42740,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":44380,"parent_record_offset":44328,"ancestor_record_offset":42740,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":44352,"parent_record_offset":44328,"ancestor_record_offset":42740,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":45948,"parent_record_offset":45924,"ancestor_record_offset":42740,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":102,"record_offset":46268,"parent_record_offset":45924,"ancestor_record_offset":42740,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufwriter::BufWriter::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":102,"record_offset":46392,"parent_record_offset":46360,"ancestor_record_offset":42740,"inlinee":9748,"source_line":113,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":102,"record_offset":46792,"parent_record_offset":46768,"ancestor_record_offset":42740,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":44212,"parent_record_offset":44188,"ancestor_record_offset":42740,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":45300,"parent_record_offset":45276,"ancestor_record_offset":42740,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":102,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":42740,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":102,"record_offset":46012,"parent_record_offset":45948,"ancestor_record_offset":42740,"inlinee":5270,"source_line":218,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":102,"record_offset":46040,"parent_record_offset":45948,"ancestor_record_offset":42740,"inlinee":5271,"source_line":215,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":102,"record_offset":45984,"parent_record_offset":45948,"ancestor_record_offset":42740,"inlinee":5269,"source_line":209,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::write_image_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":102,"record_offset":44128,"parent_record_offset":44104,"ancestor_record_offset":42740,"inlinee":9744,"source_line":810,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 810
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2610
// 首条位置: {"module":102,"inlinee":9744}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":102,"record_offset":44152,"parent_record_offset":44128,"ancestor_record_offset":44128,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::progressive_blur_downsample
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 852
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25FE
// 首条位置: {"module":102,"inlinee":9726}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":102,"record_offset":22624,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":102,"record_offset":22768,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":22468,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":102,"record_offset":22852,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":102,"record_offset":22724,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":102,"record_offset":22680,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":102,"record_offset":22528,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":102,"record_offset":22500,"parent_record_offset":22352,"ancestor_record_offset":22352,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":102,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":22352,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":102,"record_offset":22592,"parent_record_offset":22528,"ancestor_record_offset":22352,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":102,"record_offset":22792,"parent_record_offset":22768,"ancestor_record_offset":22352,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":102,"record_offset":22648,"parent_record_offset":22624,"ancestor_record_offset":22352,"inlinee":8321,"source_line":49,"archive":"_pdb/A/modules/00102.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":102,"record_offset":22564,"parent_record_offset":22528,"ancestor_record_offset":22352,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00102.jsonl.gz"}
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
// 完整结构及原始类型记录: screenshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ActiveScreenshotRequest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>", "constructor": "alloc::vec::Vec::retain", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "tiny_glade::screenshot::finish_screenshot_capture::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ActiveScreenshotRequest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::BeautyShotState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::screenshot::BeautyShotState>", "constructor": "enum2$", "arguments": ["tiny_glade::screenshot::BeautyShotState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["enum2$<tiny_glade::screenshot::BeautyShotState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotCapturedCallback
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotCapturedCallback>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ScreenshotCapturedCallback"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotRequest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tiny_glade::screenshot::ScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$19::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotRequest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ScreenshotRequest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::screenshot::ScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotSink
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::screenshot::ScreenshotSink>", "constructor": "enum2$", "arguments": ["tiny_glade::screenshot::ScreenshotSink"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::screenshot::ScreenshotSink> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::screenshot::ScreenshotSink>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: screenshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: screenshot.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 65; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::ScreenshotCapturedCallback::call
// 信息 ID: 90b256b9322c1e88d38a810a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=102, inline_record=34996; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=102, inline_record=48680; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=102, inline_record=49176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::ScreenshotCapturedCallback::new
// 信息 ID: a4055874aeb1300023646a49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=102, inline_record=11868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::add_captured_screenshots_to_steam_library
// 信息 ID: 4aa466193dfa906f75aee492
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=102, inline_record=8020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure$0
// 信息 ID: e860a5e8e6285a99d9eda5f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::add_captured_screenshots_to_steam_library", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0 <- alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0> @ 0xdfa63 -> 0xe8840 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0> @ 0xdfb0e -> 0xe8840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::consume_steam_screenshot_request
// 信息 ID: f042e1b1cd6da186b9f88e19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=102, inline_record=5796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
// 信息 ID: c42951a86e07ec83ed490091
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::consume_steam_screenshot_request", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=102, inline_record=6172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::finish_screenshot_capture
// 信息 ID: 56c0ce4852f2fe53e2251ec3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x3040bb -> 0x15cb80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::finish_screenshot_capture::closure$0
// 信息 ID: 4002602a7237fa34d870fc23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::finish_screenshot_capture", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::finish_screenshot_capture::closure_env$0 <- alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; module=73, inline_record=9916; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; module=73, inline_record=10480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests
// 信息 ID: 2750e4cd94a044859daf7fa1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e416777e10a77cce54d0c1a4: void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > @ 0xa5286 -> 0x158f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> @ 0x347980 -> 0x158f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests::closure$0
// 信息 ID: 0efd04c363129869a4620a33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::gather_screenshot_requests", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=102, inline_record=7980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests::closure$1
// 信息 ID: 3d4f30f26d532cc93e7da39a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::screenshot::gather_screenshot_requests", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::gather_screenshot_requests::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1,tuple$<enum2$<core::result::Result<array$<u32,2>,alloc::boxed::Box<dyn$<core::error::Error,core::marker::Send,core::marker::Sync>,alloc::alloc::Global> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::gather_screenshot_requests::closure_env$1 <- core::ptr::drop_in_place<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1,tuple$<enum2$<core::result::Result<array$<u32,2>,alloc::boxed::Box<dyn$<core::error::Error,core::marker::Send,core::marker::Sync>,alloc::alloc::Global> > > > >; module=223, inline_record=62608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::handle_screenshot_opportunity
// 信息 ID: 0bec9d81abc68f99afa6a589
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ff4e2 -> 0x15b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ff5f0 -> 0x15b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffc2a -> 0x15b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffd9e -> 0x15b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ff9f3 -> 0x15b1a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::handle_screenshot_opportunity::closure$0
// 信息 ID: e43af4f68f658ded9a731eaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::handle_screenshot_opportunity", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$1::render<tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; module=223, inline_record=67052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::hurry_active_screenshots_on_shutdown
// 信息 ID: 48b53e1b2137ad5c5b33b3b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ac90e9b3cc6a4bf58d1b8d0c: void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>),void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>)> > @ 0xc2d3c -> 0x15cfc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>),void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>)> @ 0x347da6 -> 0x15cfc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$0::build
// 信息 ID: ed50f7dcf0f4c0a363dcb059
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$10::clone
// 信息 ID: 7b8dccb43ad1e68f88320af5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=102, inline_record=38452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$1::drop
// 信息 ID: 87878241b8faea62019c031d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotCapturedCallback>; module=234, inline_record=769944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$6::eq
// 信息 ID: b36b355890b560c4ef4bdd37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::handle_screenshot_opportunity; module=102, inline_record=22116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::next_requested_screenshot_stage
// 信息 ID: 0bb7c3e39ab07eab3de361b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=26012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::progressive_blur_downsample
// 信息 ID: 830c5b8123a3d2c853066397
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::handle_screenshot_opportunity; module=102, inline_record=22352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::read_back_screenshot
// 信息 ID: 726c27d7a2758a136339b70b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x15cea2 -> 0x15d2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x15ceb5 -> 0x15d2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image
// 信息 ID: f6a1943fa658ed7da4113fd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=102, inline_record=35488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image::closure$0
// 信息 ID: 1544091b4543ec5dac610a8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::write_image", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- std::thread::Builder::spawn<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::screenshot::write_image::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::screenshot::write_image::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; module=14, inline_record=660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image::closure$1
// 信息 ID: 10c5152fbaa7369a9f7794b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::screenshot::write_image", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$1 <- alloc::vec::Vec::retain<std::thread::JoinHandle<tuple$<> >,alloc::alloc::Global,tiny_glade::screenshot::write_image::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<std::thread::JoinHandle<tuple$<> >,alloc::alloc::Global,tiny_glade::screenshot::write_image::closure_env$1>; module=73, inline_record=8664; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<std::thread::JoinHandle<tuple$<> >,alloc::alloc::Global,tiny_glade::screenshot::write_image::closure_env$1>; module=73, inline_record=9004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_impl
// 信息 ID: da2c3b8b1d6230d595026233
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=102, inline_record=42740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_impl::closure$0
// 信息 ID: 2c334aacdf8afde5769b77d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::write_image_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=102, inline_record=44128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_with_callback_impl
// 信息 ID: d9656223db7091876d474b48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0x1a9fc -> 0x15e450 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::read_back_screenshot @ 0x15d734 -> 0x15e450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
