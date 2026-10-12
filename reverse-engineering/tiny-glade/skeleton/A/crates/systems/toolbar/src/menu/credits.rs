// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/menu/credits.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1e17899dbec946310e251dc95dbb38fafa54d011ff48a628c33cc02ab9e8f4c7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: credits.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1018, 2421, 3655
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"PDB_registry_path_only","version":"1.19.0"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::CRATE_CREDITS::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14474
// 首条位置: {"module":2421,"inlinee":83060}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2421,"record_offset":548,"parent_record_offset":432,"ancestor_record_offset":432,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2421,"record_offset":488,"parent_record_offset":432,"ancestor_record_offset":432,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2421,"record_offset":460,"parent_record_offset":432,"ancestor_record_offset":432,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":432,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":432,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":872,"parent_record_offset":844,"ancestor_record_offset":432,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":432,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":432,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":988,"parent_record_offset":816,"ancestor_record_offset":432,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":432,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":432,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":672,"parent_record_offset":632,"ancestor_record_offset":432,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":432,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":632,"parent_record_offset":604,"ancestor_record_offset":432,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":2421,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":432,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2421,"record_offset":788,"parent_record_offset":756,"ancestor_record_offset":432,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2421,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":432,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2421,"record_offset":1024,"parent_record_offset":756,"ancestor_record_offset":432,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":520,"parent_record_offset":488,"ancestor_record_offset":432,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":432,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2421,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":432,"inlinee":83062,"source_line":21,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14476
// 首条位置: {"module":2421,"inlinee":83062}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":2421,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":580,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":580,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":872,"parent_record_offset":844,"ancestor_record_offset":580,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":580,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":580,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":988,"parent_record_offset":816,"ancestor_record_offset":580,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":580,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":580,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":672,"parent_record_offset":632,"ancestor_record_offset":580,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":580,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":632,"parent_record_offset":604,"ancestor_record_offset":580,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2421,"record_offset":788,"parent_record_offset":756,"ancestor_record_offset":580,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2421,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":580,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2421,"record_offset":1024,"parent_record_offset":756,"ancestor_record_offset":580,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":580,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2421,"record_offset":1772,"rva":22977104,"va":5391686224}
// 编译实例 RVAs: 0x15e9a50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=7
//     target_aliases=9 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22073111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8e930affe","size":5,"soft_fail":false,"target_va":5369613520,"va":5391686626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22071798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e80a36affe","size":5,"soft_fail":false,"target_va":5369616080,"va":5391687873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22071419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e88537affe","size":5,"soft_fail":false,"target_va":5369616480,"va":5391687894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22068706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e81e42affe","size":5,"soft_fail":false,"target_va":5369617648,"va":5391686349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18938540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e85405dffe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391688247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a57c70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12134624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e820d746ff","size":5,"soft_fail":false,"target_va":5379554416,"va":5391689035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a581c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12131128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8c8e446ff","size":5,"soft_fail":false,"target_va":5379555776,"va":5391686899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12125528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8a8fa46ff","size":5,"soft_fail":false,"target_va":5379561040,"va":5391686563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12124745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8b7fd46ff","size":5,"soft_fail":false,"target_va":5379561744,"va":5391686484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12124682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8f6fd46ff","size":5,"soft_fail":false,"target_va":5379563248,"va":5391687925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5b4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12117618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e88e1947ff","size":5,"soft_fail":false,"target_va":5379568848,"va":5391686461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12115796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8ac2047ff","size":5,"soft_fail":false,"target_va":5379572176,"va":5391687967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12114905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8272447ff","size":5,"soft_fail":false,"target_va":5379573040,"va":5391687940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-20377615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8f10fc9fe","size":5,"soft_fail":false,"target_va":5371308800,"va":5391686410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t11289756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e89c44ac00","size":5,"soft_fail":false,"target_va":5402978880,"va":5391689119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15644025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8874a11ff","size":5,"soft_fail":false,"target_va":5376043968,"va":5391687988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18526993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8ef4ce5fe","size":5,"soft_fail":false,"target_va":5373159392,"va":5391686380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18527047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391686224,"raw_hex":"e8b94ce5fe","size":5,"soft_fail":false,"target_va":5373160864,"va":5391687906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2421,"record_offset":1948,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":8464,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen_without_block_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":2004,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":50926,"source_line":343,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_relative
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":4560,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":8027,"source_line":377,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":8720,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":7454,"source_line":349,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":8408,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":17396,"source_line":238,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":10008,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":2376,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":3440,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2421,"record_offset":7232,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":17226,"source_line":226,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2421,"record_offset":7172,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":17224,"source_line":214,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2421,"record_offset":7112,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2421,"record_offset":6912,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$24::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2421,"record_offset":7292,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":50904,"source_line":138,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::background_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2421,"record_offset":2228,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":83064,"source_line":296,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::max_scroll_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2421,"record_offset":2288,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":83065,"source_line":301,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2421,"record_offset":2348,"parent_record_offset":1772,"ancestor_record_offset":1772,"inlinee":83066,"source_line":306,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":1772,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":8592,"parent_record_offset":8536,"ancestor_record_offset":1772,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":8564,"parent_record_offset":8536,"ancestor_record_offset":1772,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":2156,"parent_record_offset":2124,"ancestor_record_offset":1772,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":8536,"parent_record_offset":8512,"ancestor_record_offset":1772,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2421,"record_offset":2124,"parent_record_offset":2092,"ancestor_record_offset":1772,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2421,"record_offset":2092,"parent_record_offset":2028,"ancestor_record_offset":1772,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2421,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1772,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2421,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":1772,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2421,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":1772,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2421,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":1772,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2421,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1772,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2421,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":1772,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2421,"record_offset":3408,"parent_record_offset":2376,"ancestor_record_offset":1772,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":8488,"parent_record_offset":8464,"ancestor_record_offset":1772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2421,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":1772,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2421,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":1772,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":2188,"parent_record_offset":2092,"ancestor_record_offset":1772,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2421,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":1772,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2421,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":1772,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2421,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":1772,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2421,"record_offset":3144,"parent_record_offset":2924,"ancestor_record_offset":1772,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2421,"record_offset":2488,"parent_record_offset":2456,"ancestor_record_offset":1772,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":1772,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":3032,"parent_record_offset":2924,"ancestor_record_offset":1772,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":1772,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":3352,"parent_record_offset":2564,"ancestor_record_offset":1772,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":1772,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":1772,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":2516,"parent_record_offset":2456,"ancestor_record_offset":1772,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":1772,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2421,"record_offset":2804,"parent_record_offset":2776,"ancestor_record_offset":1772,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2421,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":1772,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2421,"record_offset":2892,"parent_record_offset":2776,"ancestor_record_offset":1772,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2421,"record_offset":2776,"parent_record_offset":2564,"ancestor_record_offset":1772,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":2608,"parent_record_offset":2564,"ancestor_record_offset":1772,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2421,"record_offset":2924,"parent_record_offset":2564,"ancestor_record_offset":1772,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2421,"record_offset":2636,"parent_record_offset":2564,"ancestor_record_offset":1772,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2421,"record_offset":2720,"parent_record_offset":2564,"ancestor_record_offset":1772,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2421,"record_offset":3260,"parent_record_offset":2564,"ancestor_record_offset":1772,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2421,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":1772,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":1772,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":7020,"parent_record_offset":6964,"ancestor_record_offset":1772,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":6964,"parent_record_offset":6912,"ancestor_record_offset":1772,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":2256,"parent_record_offset":2228,"ancestor_record_offset":1772,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":7328,"parent_record_offset":7292,"ancestor_record_offset":1772,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2421,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":1772,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":2064,"parent_record_offset":2028,"ancestor_record_offset":1772,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":1772,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":6992,"parent_record_offset":6964,"ancestor_record_offset":1772,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":1772,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":1772,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2421,"record_offset":7076,"parent_record_offset":6964,"ancestor_record_offset":1772,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6D3
// 首条位置: {"module":1018,"inlinee":50899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1018,"record_offset":83056,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1018,"record_offset":72064,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":14514,"source_line":271,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1018,"record_offset":81856,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1018,"record_offset":81828,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1018,"record_offset":73852,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":83564,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1018,"record_offset":74968,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":19447,"source_line":1264,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1018,"record_offset":75844,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":9155,"source_line":187,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1018,"record_offset":75632,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":7369,"source_line":2217,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::trim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1018,"record_offset":75360,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":6115,"source_line":1946,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1018,"record_offset":75052,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":1018,"record_offset":71820,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":83000,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":17189,"source_line":252,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":73796,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":17396,"source_line":238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::prev_computed_layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":81772,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":17399,"source_line":264,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":75872,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":75984,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":79128,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":77568,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":17226,"source_line":226,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":77508,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":17224,"source_line":214,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":77448,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":80420,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":17225,"source_line":220,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label_with_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":77276,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":50900,"source_line":55,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$23::deref
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":80656,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":50906,"source_line":132,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$24::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":77628,"parent_record_offset":71620,"ancestor_record_offset":71620,"inlinee":50904,"source_line":138,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":83348,"parent_record_offset":83324,"ancestor_record_offset":71620,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":83372,"parent_record_offset":83348,"ancestor_record_offset":71620,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":73744,"parent_record_offset":73720,"ancestor_record_offset":71620,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":83324,"parent_record_offset":83288,"ancestor_record_offset":71620,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":73720,"parent_record_offset":73664,"ancestor_record_offset":71620,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":72528,"parent_record_offset":72500,"ancestor_record_offset":71620,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":73692,"parent_record_offset":73664,"ancestor_record_offset":71620,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":72500,"parent_record_offset":72468,"ancestor_record_offset":71620,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":72408,"parent_record_offset":72364,"ancestor_record_offset":71620,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":72468,"parent_record_offset":72440,"ancestor_record_offset":71620,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":83288,"parent_record_offset":83260,"ancestor_record_offset":71620,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":83260,"parent_record_offset":83232,"ancestor_record_offset":71620,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":83444,"parent_record_offset":83260,"ancestor_record_offset":71620,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":73664,"parent_record_offset":73636,"ancestor_record_offset":71620,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1018,"record_offset":83196,"parent_record_offset":83168,"ancestor_record_offset":71620,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1018,"record_offset":83168,"parent_record_offset":83140,"ancestor_record_offset":71620,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1018,"record_offset":83112,"parent_record_offset":83084,"ancestor_record_offset":71620,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1018,"record_offset":83140,"parent_record_offset":83112,"ancestor_record_offset":71620,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1018,"record_offset":83084,"parent_record_offset":83056,"ancestor_record_offset":71620,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1018,"record_offset":72140,"parent_record_offset":72064,"ancestor_record_offset":71620,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1018,"record_offset":72224,"parent_record_offset":72064,"ancestor_record_offset":71620,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1018,"record_offset":72320,"parent_record_offset":72288,"ancestor_record_offset":71620,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1018,"record_offset":73060,"parent_record_offset":72868,"ancestor_record_offset":71620,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1018,"record_offset":72256,"parent_record_offset":72224,"ancestor_record_offset":71620,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1018,"record_offset":72580,"parent_record_offset":72320,"ancestor_record_offset":71620,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1018,"record_offset":72440,"parent_record_offset":72320,"ancestor_record_offset":71620,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1018,"record_offset":83232,"parent_record_offset":83196,"ancestor_record_offset":71620,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1018,"record_offset":72288,"parent_record_offset":72256,"ancestor_record_offset":71620,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":74204,"parent_record_offset":73988,"ancestor_record_offset":71620,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":76272,"parent_record_offset":76248,"ancestor_record_offset":71620,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":76356,"parent_record_offset":76332,"ancestor_record_offset":71620,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":76296,"parent_record_offset":76272,"ancestor_record_offset":71620,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1018,"record_offset":73900,"parent_record_offset":73876,"ancestor_record_offset":71620,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1018,"record_offset":75228,"parent_record_offset":75140,"ancestor_record_offset":71620,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":76468,"parent_record_offset":76444,"ancestor_record_offset":71620,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1018,"record_offset":74356,"parent_record_offset":74328,"ancestor_record_offset":71620,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":81796,"parent_record_offset":81772,"ancestor_record_offset":71620,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":72676,"parent_record_offset":72648,"ancestor_record_offset":71620,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":77020,"parent_record_offset":75984,"ancestor_record_offset":71620,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1018,"record_offset":75312,"parent_record_offset":75284,"ancestor_record_offset":71620,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1018,"record_offset":83484,"parent_record_offset":83196,"ancestor_record_offset":71620,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":73588,"parent_record_offset":72064,"ancestor_record_offset":71620,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":72364,"parent_record_offset":72320,"ancestor_record_offset":71620,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":76716,"parent_record_offset":76692,"ancestor_record_offset":71620,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":76608,"parent_record_offset":76584,"ancestor_record_offset":71620,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":83400,"parent_record_offset":83372,"ancestor_record_offset":71620,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1018,"record_offset":74228,"parent_record_offset":74204,"ancestor_record_offset":71620,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1018,"record_offset":74252,"parent_record_offset":74228,"ancestor_record_offset":71620,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1018,"record_offset":74580,"parent_record_offset":74548,"ancestor_record_offset":71620,"inlinee":5504,"source_line":2585,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1018,"record_offset":74032,"parent_record_offset":73988,"ancestor_record_offset":71620,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1018,"record_offset":75716,"parent_record_offset":75688,"ancestor_record_offset":71620,"inlinee":5503,"source_line":2554,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1018,"record_offset":75196,"parent_record_offset":75172,"ancestor_record_offset":71620,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1018,"record_offset":74088,"parent_record_offset":74032,"ancestor_record_offset":71620,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\memchr.rs
//     evidence={"module":1018,"record_offset":74128,"parent_record_offset":73988,"ancestor_record_offset":71620,"inlinee":5066,"source_line":27,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\memchr.rs
//     evidence={"module":1018,"record_offset":74160,"parent_record_offset":74128,"ancestor_record_offset":71620,"inlinee":5067,"source_line":38,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1018,"record_offset":72716,"parent_record_offset":72064,"ancestor_record_offset":71620,"inlinee":12626,"source_line":507,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1018,"record_offset":75140,"parent_record_offset":75112,"ancestor_record_offset":71620,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1018,"record_offset":72168,"parent_record_offset":72064,"ancestor_record_offset":71620,"inlinee":14515,"source_line":1882,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1018,"record_offset":74452,"parent_record_offset":74412,"ancestor_record_offset":71620,"inlinee":22131,"source_line":2244,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$5::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1018,"record_offset":74412,"parent_record_offset":74384,"ancestor_record_offset":71620,"inlinee":22130,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$6::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1018,"record_offset":74384,"parent_record_offset":74356,"ancestor_record_offset":71620,"inlinee":22128,"source_line":104,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1018,"record_offset":72648,"parent_record_offset":72624,"ancestor_record_offset":71620,"inlinee":14519,"source_line":1054,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::SplitInternal::get_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1018,"record_offset":74292,"parent_record_offset":73952,"ancestor_record_offset":71620,"inlinee":16678,"source_line":637,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::SplitInternal::next_inclusive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1018,"record_offset":73952,"parent_record_offset":73924,"ancestor_record_offset":71620,"inlinee":22126,"source_line":670,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$30::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1018,"record_offset":73876,"parent_record_offset":73852,"ancestor_record_offset":71620,"inlinee":22123,"source_line":1164,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$45::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1018,"record_offset":73924,"parent_record_offset":73900,"ancestor_record_offset":71620,"inlinee":22125,"source_line":1417,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1018,"record_offset":72624,"parent_record_offset":72064,"ancestor_record_offset":71620,"inlinee":14517,"source_line":506,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1018,"record_offset":73988,"parent_record_offset":73952,"ancestor_record_offset":71620,"inlinee":16677,"source_line":418,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1018,"record_offset":72192,"parent_record_offset":72168,"ancestor_record_offset":71620,"inlinee":7267,"source_line":956,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1018,"record_offset":75688,"parent_record_offset":75660,"ancestor_record_offset":71620,"inlinee":7373,"source_line":962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1018,"record_offset":74548,"parent_record_offset":74516,"ancestor_record_offset":71620,"inlinee":12627,"source_line":1005,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1018,"record_offset":75660,"parent_record_offset":75632,"ancestor_record_offset":71620,"inlinee":7372,"source_line":994,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1018,"record_offset":74516,"parent_record_offset":74484,"ancestor_record_offset":71620,"inlinee":22133,"source_line":1011,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::strip_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1018,"record_offset":74484,"parent_record_offset":74452,"ancestor_record_offset":71620,"inlinee":22132,"source_line":597,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1018,"record_offset":75112,"parent_record_offset":75080,"ancestor_record_offset":71620,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1018,"record_offset":75260,"parent_record_offset":75112,"ancestor_record_offset":71620,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1018,"record_offset":75080,"parent_record_offset":75052,"ancestor_record_offset":71620,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1018,"record_offset":72740,"parent_record_offset":72716,"ancestor_record_offset":71620,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1018,"record_offset":75448,"parent_record_offset":75416,"ancestor_record_offset":71620,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1018,"record_offset":75416,"parent_record_offset":75388,"ancestor_record_offset":71620,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":71984,"parent_record_offset":71960,"ancestor_record_offset":71620,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":72008,"parent_record_offset":71984,"ancestor_record_offset":71620,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":76780,"parent_record_offset":76756,"ancestor_record_offset":71620,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":80788,"parent_record_offset":80752,"ancestor_record_offset":71620,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":80812,"parent_record_offset":80788,"ancestor_record_offset":71620,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":76068,"parent_record_offset":76044,"ancestor_record_offset":71620,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":76044,"parent_record_offset":76020,"ancestor_record_offset":71620,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":76756,"parent_record_offset":76536,"ancestor_record_offset":71620,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":76100,"parent_record_offset":76068,"ancestor_record_offset":71620,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76668,"parent_record_offset":76644,"ancestor_record_offset":71620,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76644,"parent_record_offset":76536,"ancestor_record_offset":71620,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76584,"parent_record_offset":76560,"ancestor_record_offset":71620,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76964,"parent_record_offset":76176,"ancestor_record_offset":71620,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76560,"parent_record_offset":76536,"ancestor_record_offset":71620,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76152,"parent_record_offset":76128,"ancestor_record_offset":71620,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":80868,"parent_record_offset":80812,"ancestor_record_offset":71620,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76128,"parent_record_offset":76068,"ancestor_record_offset":71620,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76176,"parent_record_offset":76152,"ancestor_record_offset":71620,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":76416,"parent_record_offset":76388,"ancestor_record_offset":71620,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":76444,"parent_record_offset":76416,"ancestor_record_offset":71620,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":76504,"parent_record_offset":76388,"ancestor_record_offset":71620,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":76388,"parent_record_offset":76176,"ancestor_record_offset":71620,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76220,"parent_record_offset":76176,"ancestor_record_offset":71620,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":76536,"parent_record_offset":76176,"ancestor_record_offset":71620,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":76248,"parent_record_offset":76176,"ancestor_record_offset":71620,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":76332,"parent_record_offset":76176,"ancestor_record_offset":71620,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":76872,"parent_record_offset":76176,"ancestor_record_offset":71620,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":1018,"record_offset":71960,"parent_record_offset":71936,"ancestor_record_offset":71620,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":1018,"record_offset":71848,"parent_record_offset":71820,"ancestor_record_offset":71620,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":1018,"record_offset":71936,"parent_record_offset":71904,"ancestor_record_offset":71620,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":1018,"record_offset":71876,"parent_record_offset":71848,"ancestor_record_offset":71620,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":1018,"record_offset":71904,"parent_record_offset":71876,"ancestor_record_offset":71620,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":80752,"parent_record_offset":80720,"ancestor_record_offset":71620,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":76020,"parent_record_offset":75984,"ancestor_record_offset":71620,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":80720,"parent_record_offset":80688,"ancestor_record_offset":71620,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77356,"parent_record_offset":77300,"ancestor_record_offset":71620,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77300,"parent_record_offset":77276,"ancestor_record_offset":71620,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=10
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77476,"parent_record_offset":77448,"ancestor_record_offset":71620,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":80688,"parent_record_offset":80656,"ancestor_record_offset":71620,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77668,"parent_record_offset":77628,"ancestor_record_offset":71620,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":73820,"parent_record_offset":73796,"ancestor_record_offset":71620,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":83024,"parent_record_offset":83000,"ancestor_record_offset":71620,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":76828,"parent_record_offset":76804,"ancestor_record_offset":71620,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":75920,"parent_record_offset":75896,"ancestor_record_offset":71620,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":75948,"parent_record_offset":75896,"ancestor_record_offset":71620,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":75896,"parent_record_offset":75872,"ancestor_record_offset":71620,"inlinee":5594,"source_line":135,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label_with_id::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":77412,"parent_record_offset":77300,"ancestor_record_offset":71620,"inlinee":50902,"source_line":56,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6D0
// 首条位置: {"module":1018,"inlinee":50896}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1018,"record_offset":69376,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1018,"record_offset":70848,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":68972,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1018,"record_offset":69000,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":69084,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":68832,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":68632,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1018,"record_offset":69028,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1018,"record_offset":68888,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":68572,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":17372,"source_line":248,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::deref
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":67444,"parent_record_offset":67324,"ancestor_record_offset":67324,"inlinee":17184,"source_line":132,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":68784,"parent_record_offset":68760,"ancestor_record_offset":67324,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1018,"record_offset":68760,"parent_record_offset":68704,"ancestor_record_offset":67324,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1018,"record_offset":69424,"parent_record_offset":69400,"ancestor_record_offset":67324,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":68732,"parent_record_offset":68704,"ancestor_record_offset":67324,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1018,"record_offset":68704,"parent_record_offset":68680,"ancestor_record_offset":67324,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1018,"record_offset":70876,"parent_record_offset":70848,"ancestor_record_offset":67324,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":68344,"parent_record_offset":68320,"ancestor_record_offset":67324,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":67812,"parent_record_offset":67788,"ancestor_record_offset":67324,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":67896,"parent_record_offset":67872,"ancestor_record_offset":67324,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":67836,"parent_record_offset":67812,"ancestor_record_offset":67324,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":68008,"parent_record_offset":67984,"ancestor_record_offset":67324,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":70980,"parent_record_offset":70956,"ancestor_record_offset":67324,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":68856,"parent_record_offset":68832,"ancestor_record_offset":67324,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":69400,"parent_record_offset":69376,"ancestor_record_offset":67324,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":68656,"parent_record_offset":68632,"ancestor_record_offset":67324,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":68256,"parent_record_offset":68232,"ancestor_record_offset":67324,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":68148,"parent_record_offset":68124,"ancestor_record_offset":67324,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":68912,"parent_record_offset":68888,"ancestor_record_offset":67324,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":68936,"parent_record_offset":68912,"ancestor_record_offset":67324,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":68320,"parent_record_offset":68296,"ancestor_record_offset":67324,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":67580,"parent_record_offset":67540,"ancestor_record_offset":67324,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":67608,"parent_record_offset":67580,"ancestor_record_offset":67324,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":68296,"parent_record_offset":68076,"ancestor_record_offset":67324,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":67640,"parent_record_offset":67608,"ancestor_record_offset":67324,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":68208,"parent_record_offset":68184,"ancestor_record_offset":67324,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":68184,"parent_record_offset":68076,"ancestor_record_offset":67324,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":68124,"parent_record_offset":68100,"ancestor_record_offset":67324,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":68504,"parent_record_offset":67716,"ancestor_record_offset":67324,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":68100,"parent_record_offset":68076,"ancestor_record_offset":67324,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":67692,"parent_record_offset":67668,"ancestor_record_offset":67324,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":67668,"parent_record_offset":67608,"ancestor_record_offset":67324,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":67716,"parent_record_offset":67692,"ancestor_record_offset":67324,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":67956,"parent_record_offset":67928,"ancestor_record_offset":67324,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":67984,"parent_record_offset":67956,"ancestor_record_offset":67324,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":68044,"parent_record_offset":67928,"ancestor_record_offset":67324,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":67928,"parent_record_offset":67716,"ancestor_record_offset":67324,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":67760,"parent_record_offset":67716,"ancestor_record_offset":67324,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":68076,"parent_record_offset":67716,"ancestor_record_offset":67324,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":67788,"parent_record_offset":67716,"ancestor_record_offset":67324,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":67872,"parent_record_offset":67716,"ancestor_record_offset":67324,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":68412,"parent_record_offset":67716,"ancestor_record_offset":67324,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":67540,"parent_record_offset":67508,"ancestor_record_offset":67324,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":67508,"parent_record_offset":67476,"ancestor_record_offset":67324,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":67476,"parent_record_offset":67444,"ancestor_record_offset":67324,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":68600,"parent_record_offset":68572,"ancestor_record_offset":67324,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":68368,"parent_record_offset":68344,"ancestor_record_offset":67324,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2421,"record_offset":11760,"rva":22980304,"va":5391689424}
// 编译实例 RVAs: 0x15ea6d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22957846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e8eab0a1fe","size":5,"soft_fail":false,"target_va":5368732256,"va":5391690097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18940132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e81cffdefe","size":5,"soft_fail":false,"target_va":5372749696,"va":5391689823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18940009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e897ffdefe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391689716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5d0e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12114012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e8a42747ff","size":5,"soft_fail":false,"target_va":5379576032,"va":5391690039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b5dfa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5716106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e88a385700","size":5,"soft_fail":false,"target_va":5397405600,"va":5391689489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11288220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e89c3eac00","size":5,"soft_fail":false,"target_va":5402978144,"va":5391689919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14159759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e871f027ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5391689642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11850488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e8f8d2b400","size":5,"soft_fail":false,"target_va":5403540144,"va":5391689651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5716386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391689424,"raw_hex":"e8a2395700","size":5,"soft_fail":false,"target_va":5397406192,"va":5391689801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5391689424,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5391689687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":2421,"record_offset":12440,"parent_record_offset":11760,"ancestor_record_offset":11760,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2421,"record_offset":11916,"parent_record_offset":11760,"ancestor_record_offset":11760,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2421,"record_offset":12412,"parent_record_offset":11760,"ancestor_record_offset":11760,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":12724,"parent_record_offset":12700,"ancestor_record_offset":11760,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":12748,"parent_record_offset":12724,"ancestor_record_offset":11760,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":11760,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":12700,"parent_record_offset":12672,"ancestor_record_offset":11760,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":12196,"parent_record_offset":12112,"ancestor_record_offset":11760,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2421,"record_offset":12112,"parent_record_offset":12084,"ancestor_record_offset":11760,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2421,"record_offset":12000,"parent_record_offset":11960,"ancestor_record_offset":11760,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2421,"record_offset":12048,"parent_record_offset":12024,"ancestor_record_offset":11760,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":12672,"parent_record_offset":12644,"ancestor_record_offset":11760,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":12644,"parent_record_offset":12616,"ancestor_record_offset":11760,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":12820,"parent_record_offset":12644,"ancestor_record_offset":11760,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":12580,"parent_record_offset":12552,"ancestor_record_offset":11760,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":12552,"parent_record_offset":12524,"ancestor_record_offset":11760,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":12496,"parent_record_offset":12468,"ancestor_record_offset":11760,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":12524,"parent_record_offset":12496,"ancestor_record_offset":11760,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2421,"record_offset":12468,"parent_record_offset":12440,"ancestor_record_offset":11760,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2421,"record_offset":12616,"parent_record_offset":12580,"ancestor_record_offset":11760,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2421,"record_offset":12140,"parent_record_offset":12112,"ancestor_record_offset":11760,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2421,"record_offset":12880,"parent_record_offset":12856,"ancestor_record_offset":11760,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2421,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11760,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2421,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":11760,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2421,"record_offset":12856,"parent_record_offset":12580,"ancestor_record_offset":11760,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":12084,"parent_record_offset":11960,"ancestor_record_offset":11760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":12776,"parent_record_offset":12748,"ancestor_record_offset":11760,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::spawn_third_party_license_http_server::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2421,"record_offset":11960,"parent_record_offset":11916,"ancestor_record_offset":11760,"inlinee":83068,"source_line":230,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1447C
// 首条位置: {"module":2421,"inlinee":83068}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2421,"record_offset":12000,"parent_record_offset":11960,"ancestor_record_offset":11960,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":12084,"parent_record_offset":11960,"ancestor_record_offset":11960,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":11960,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":12196,"parent_record_offset":12112,"ancestor_record_offset":11960,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2421,"record_offset":12112,"parent_record_offset":12084,"ancestor_record_offset":11960,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2421,"record_offset":12048,"parent_record_offset":12024,"ancestor_record_offset":11960,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2421,"record_offset":12140,"parent_record_offset":12112,"ancestor_record_offset":11960,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2421,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11960,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2421,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":11960,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A8EA
// 首条位置: {"module":3655,"inlinee":108778}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3655,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/03655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3655,"record_offset":816,"parent_record_offset":780,"ancestor_record_offset":720,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3655,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":720,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3655,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":720,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03655.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::spawn_third_party_license_http_server::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":3655,"record_offset":780,"parent_record_offset":748,"ancestor_record_offset":720,"inlinee":108780,"source_line":238,"archive":"_pdb/A/modules/03655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A8EC
// 首条位置: {"module":3655,"inlinee":108780}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3655,"record_offset":816,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3655,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":780,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03655.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3655,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":780,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$0::scroll<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1018,"record_offset":113468,"rva":10859728,"va":5379568848}
// 编译实例 RVAs: 0xa5b4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9955488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568848,"raw_hex":"e8601768ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5379569003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9952824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568848,"raw_hex":"e8c82168ff","size":5,"soft_fail":false,"target_va":5369616080,"va":5379568899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9952448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568848,"raw_hex":"e8402368ff","size":5,"soft_fail":false,"target_va":5369616480,"va":5379568923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568848,"raw_hex":"e8c9e3ffff","size":5,"soft_fail":false,"target_va":5379561744,"va":5379568962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23409412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568848,"raw_hex":"e804336501","size":5,"soft_fail":false,"target_va":5402978880,"va":5379569463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6408075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568848,"raw_hex":"e875389eff","size":5,"soft_fail":false,"target_va":5373160864,"va":5379568934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":113604,"parent_record_offset":113468,"ancestor_record_offset":113468,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::grab_focus
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":114780,"parent_record_offset":113468,"ancestor_record_offset":113468,"inlinee":17412,"source_line":839,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":113716,"parent_record_offset":113468,"ancestor_record_offset":113468,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":114532,"parent_record_offset":114508,"ancestor_record_offset":113468,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":114000,"parent_record_offset":113976,"ancestor_record_offset":113468,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":114084,"parent_record_offset":114060,"ancestor_record_offset":113468,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":114024,"parent_record_offset":114000,"ancestor_record_offset":113468,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":114196,"parent_record_offset":114172,"ancestor_record_offset":113468,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":114748,"parent_record_offset":113716,"ancestor_record_offset":113468,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":114444,"parent_record_offset":114420,"ancestor_record_offset":113468,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":114336,"parent_record_offset":114312,"ancestor_record_offset":113468,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":114508,"parent_record_offset":114484,"ancestor_record_offset":113468,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":113796,"parent_record_offset":113772,"ancestor_record_offset":113468,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":113772,"parent_record_offset":113748,"ancestor_record_offset":113468,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":114484,"parent_record_offset":114264,"ancestor_record_offset":113468,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":113828,"parent_record_offset":113796,"ancestor_record_offset":113468,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":114396,"parent_record_offset":114372,"ancestor_record_offset":113468,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":114372,"parent_record_offset":114264,"ancestor_record_offset":113468,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":114312,"parent_record_offset":114288,"ancestor_record_offset":113468,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":114692,"parent_record_offset":113904,"ancestor_record_offset":113468,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":114288,"parent_record_offset":114264,"ancestor_record_offset":113468,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":113880,"parent_record_offset":113856,"ancestor_record_offset":113468,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":113856,"parent_record_offset":113796,"ancestor_record_offset":113468,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":113904,"parent_record_offset":113880,"ancestor_record_offset":113468,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":114144,"parent_record_offset":114116,"ancestor_record_offset":113468,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":114172,"parent_record_offset":114144,"ancestor_record_offset":113468,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":114232,"parent_record_offset":114116,"ancestor_record_offset":113468,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":114116,"parent_record_offset":113904,"ancestor_record_offset":113468,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":113948,"parent_record_offset":113904,"ancestor_record_offset":113468,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":114264,"parent_record_offset":113904,"ancestor_record_offset":113468,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":113976,"parent_record_offset":113904,"ancestor_record_offset":113468,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":114060,"parent_record_offset":113904,"ancestor_record_offset":113468,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":114600,"parent_record_offset":113904,"ancestor_record_offset":113468,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":113748,"parent_record_offset":113716,"ancestor_record_offset":113468,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":113656,"parent_record_offset":113604,"ancestor_record_offset":113468,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1018,"record_offset":115804,"parent_record_offset":114780,"ancestor_record_offset":113468,"inlinee":17293,"source_line":609,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":114556,"parent_record_offset":114532,"ancestor_record_offset":113468,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":113632,"parent_record_offset":113604,"ancestor_record_offset":113468,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":113680,"parent_record_offset":113656,"ancestor_record_offset":113468,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$7::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1447E
// 首条位置: {"module":2421,"inlinee":83070}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$1::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6E1
// 首条位置: {"module":1018,"inlinee":50913}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":92300,"parent_record_offset":92276,"ancestor_record_offset":92276,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":92348,"parent_record_offset":92324,"ancestor_record_offset":92276,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":92324,"parent_record_offset":92300,"ancestor_record_offset":92276,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":92384,"parent_record_offset":92348,"ancestor_record_offset":92276,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":92408,"parent_record_offset":92384,"ancestor_record_offset":92276,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":92440,"parent_record_offset":92348,"ancestor_record_offset":92276,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::background_color
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14478
// 首条位置: {"module":2421,"inlinee":83064}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":2256,"parent_record_offset":2228,"ancestor_record_offset":2228,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::max_scroll_pos
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14479
// 首条位置: {"module":2421,"inlinee":83065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":2288,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::offset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 306
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1447A
// 首条位置: {"module":2421,"inlinee":83066}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$3::on_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2421,"record_offset":12940,"rva":22980992,"va":5391690112}
// 编译实例 RVAs: 0x15ea980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$3::prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 319
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2421,"record_offset":13056,"rva":22981040,"va":5391690160}
// 编译实例 RVAs: 0x15ea9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_mut
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14749949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391690160,"raw_hex":"e803ef1eff","size":5,"soft_fail":false,"target_va":5376940320,"va":5391690264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2421,"record_offset":13276,"parent_record_offset":13056,"ancestor_record_offset":13056,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2421,"record_offset":13192,"parent_record_offset":13056,"ancestor_record_offset":13056,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2421,"record_offset":13220,"parent_record_offset":13056,"ancestor_record_offset":13056,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":13304,"parent_record_offset":13056,"ancestor_record_offset":13056,"inlinee":17206,"source_line":977,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$3::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 375
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2421,"record_offset":13336,"rva":22981280,"va":5391690400}
// 编译实例 RVAs: 0x15eaaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18940901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391690400,"raw_hex":"e81bfcdefe","size":5,"soft_fail":false,"target_va":5372749712,"va":5391690608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056c370
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17295214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391690400,"raw_hex":"e89218f8fe","size":5,"soft_fail":false,"target_va":5374395248,"va":5391690457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12093426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391690400,"raw_hex":"e80e7847ff","size":5,"soft_fail":false,"target_va":5379597488,"va":5391690909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::rect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15653249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391690400,"raw_hex":"e87f2611ff","size":5,"soft_fail":false,"target_va":5376037504,"va":5391690748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12070716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391690400,"raw_hex":"e8c4d047ff","size":5,"soft_fail":false,"target_va":5379619792,"va":5391690503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12070693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391690400,"raw_hex":"e8dbd047ff","size":5,"soft_fail":false,"target_va":5379619760,"va":5391690448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::fill_color
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12071214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391690400,"raw_hex":"e8d2ce47ff","size":5,"soft_fail":false,"target_va":5379619264,"va":5391690473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":13476,"parent_record_offset":13336,"ancestor_record_offset":13336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2421,"record_offset":14192,"parent_record_offset":13336,"ancestor_record_offset":13336,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2421,"record_offset":14104,"parent_record_offset":13336,"ancestor_record_offset":13336,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2421,"record_offset":14016,"parent_record_offset":13336,"ancestor_record_offset":13336,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::RenderContext::draw_rect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2421,"record_offset":14280,"parent_record_offset":13336,"ancestor_record_offset":13336,"inlinee":17200,"source_line":1035,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":13760,"parent_record_offset":13736,"ancestor_record_offset":13336,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2421,"record_offset":13736,"parent_record_offset":13712,"ancestor_record_offset":13336,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":13880,"parent_record_offset":13852,"ancestor_record_offset":13336,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2421,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":13336,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2421,"record_offset":13524,"parent_record_offset":13500,"ancestor_record_offset":13336,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2421,"record_offset":14312,"parent_record_offset":14280,"ancestor_record_offset":13336,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2421,"record_offset":13908,"parent_record_offset":13880,"ancestor_record_offset":13336,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2421,"record_offset":14044,"parent_record_offset":14016,"ancestor_record_offset":13336,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2421,"record_offset":14220,"parent_record_offset":14192,"ancestor_record_offset":13336,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2421,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":13336,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2421,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":13336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2421,"record_offset":14368,"parent_record_offset":14280,"ancestor_record_offset":13336,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02421.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2421,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":13336,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/02421.jsonl.gz"}
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
// 完整结构及原始类型记录: credits.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::CRATE_CREDITS
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["toolbar::menu::credits::CRATE_CREDITS::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ScrollContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::credits::ScrollContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContext::content", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::credits::ScrollContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ScrollView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ThirdPartyLicenseServer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 31
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer>", "constructor": "core::option::Option", "arguments": ["toolbar::menu::credits::ThirdPartyLicenseServer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> >", "constructor": "enum2$", "arguments": ["core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_utils::synccell::SyncCell<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >", "constructor": "bevy_utils::synccell::SyncCell", "arguments": ["enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_utils::synccell::SyncCell<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_utils::synccell::SyncCell<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>,bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState> >", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::run_if", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>", "bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: credits.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: credits.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::system::system_param::Local<f32>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<f32>

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::CRATE_CREDITS::closure$0
// 信息 ID: a01c7ec0e506c52136cc55d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::CRATE_CREDITS", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::CRATE_CREDITS::closure_env$0 <- core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; module=2421, inline_record=432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
// 信息 ID: 881bd148f91549f21085971d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::CRATE_CREDITS", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; module=2421, inline_record=580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::background_color
// 信息 ID: f35a6bcbf895ad280a9f4a15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2421, inline_record=2228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::max_scroll_pos
// 信息 ID: b35634855a2e00ac04525f5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2421, inline_record=2288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::offset
// 信息 ID: 7321257fa914db1af068e265
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2421, inline_record=2348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$0::scroll<u64>
// 信息 ID: 6496927f2e645c8f6519ed2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::menu::credits::impl$0::scroll -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::ui @ 0x15e9b3d -> 0xa5b4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$1::create_node_content
// 信息 ID: 3ceedfd251486ea770906ed3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::credits::ScrollContent>; module=1018, inline_record=92276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$3::on_input
// 信息 ID: 530452313536e724fd53059b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$3::prepare
// 信息 ID: f74414a8db39e7a41b49810f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$3::render
// 信息 ID: c3e1ca8583196fd9ce9cfc96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$7::default
// 信息 ID: 4e8e41f1370e055ed21a5f50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server
// 信息 ID: 508160cbea0b8ea224ff9d86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1> @ 0xa57de7 -> 0x15ea6d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$0
// 信息 ID: 5b6c0978dbcc96aaab2a0bfe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::spawn_third_party_license_http_server; module=2421, inline_record=11960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1
// 信息 ID: d11d0370a4eecae89c0d6564
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- std::thread::spawn<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- std::sys_common::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; module=3655, inline_record=720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1::closure$0
// 信息 ID: ad835c456b8100c7b58677e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; module=3655, inline_record=780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui
// 信息 ID: 151ce589391e77a2f48c7ae9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc8d029f3a5cd8d5737278ec: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> @ 0xa929a5 -> 0x15e9a50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > @ 0x15de7fb -> 0x15e9a50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui::closure$0
// 信息 ID: abdd49a9c1b50bcd88586ecc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::ui::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=71620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui::closure$1
// 信息 ID: 78eedc1d02fb7cb15d1ffe25
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::ui::closure_env$1 <- toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; module=1018, inline_record=67324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
