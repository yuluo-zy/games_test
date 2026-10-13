// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/panic_reporter/panic_dump.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 85456c3e0e5a0e286f3b19f79ff68528973d4aaa53214cc413651ae1ccacf423
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_dump.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 251
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_log_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":251,"record_offset":2604,"rva":3935792,"va":5372644912}
// 编译实例 RVAs: 0x3c0e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-331203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372644912,"raw_hex":"e83df2faff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372645406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t104638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372644912,"raw_hex":"e8be980100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372645069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2526044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372644912,"raw_hex":"e85c8b2600","size":5,"soft_fail":false,"target_va":5375171056,"va":5372645007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h8068e61a17c505da (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30303432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372644912,"raw_hex":"e8c864ce01","size":5,"soft_fail":false,"target_va":5402948592,"va":5372645155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3509028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372644912,"raw_hex":"e8248b3500","size":5,"soft_fail":false,"target_va":5376154000,"va":5372644967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4750456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372644912,"raw_hex":"e8787c4800","size":5,"soft_fail":false,"target_va":5377395552,"va":5372645091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4794809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372644912,"raw_hex":"e8b9294900","size":5,"soft_fail":false,"target_va":5377439776,"va":5372644962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4830140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372644912,"raw_hex":"e8bcb34900","size":5,"soft_fail":false,"target_va":5377475184,"va":5372645039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":251,"record_offset":3848,"parent_record_offset":2604,"ancestor_record_offset":2604,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":4304,"parent_record_offset":2604,"ancestor_record_offset":2604,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":3956,"parent_record_offset":2604,"ancestor_record_offset":2604,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":251,"record_offset":2820,"parent_record_offset":2604,"ancestor_record_offset":2604,"inlinee":4754,"source_line":2066,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":2760,"parent_record_offset":2604,"ancestor_record_offset":2604,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":2604,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":3460,"parent_record_offset":3404,"ancestor_record_offset":2604,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":3432,"parent_record_offset":3404,"ancestor_record_offset":2604,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":3404,"parent_record_offset":3376,"ancestor_record_offset":2604,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":251,"record_offset":3916,"parent_record_offset":3880,"ancestor_record_offset":2604,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":251,"record_offset":3880,"parent_record_offset":3848,"ancestor_record_offset":2604,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":251,"record_offset":3020,"parent_record_offset":2820,"ancestor_record_offset":2604,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":3244,"parent_record_offset":2820,"ancestor_record_offset":2604,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":2604,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2604,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2604,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2604,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2604,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":2852,"parent_record_offset":2820,"ancestor_record_offset":2604,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":251,"record_offset":4608,"rva":3936304,"va":5372645424}
// 编译实例 RVAs: 0x3c1030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14019f730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2234821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e83be6ddff","size":5,"soft_fail":false,"target_va":5370410800,"va":5372645616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=4
//     target_aliases=89 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-331946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e856effaff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372646149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t104124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e8bc960100","size":5,"soft_fail":false,"target_va":5372749696,"va":5372645567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t104051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e873960100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372645656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2525086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e89e872600","size":5,"soft_fail":false,"target_va":5375171056,"va":5372645965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30331611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e8dbd2ce01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372646528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t30299740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e85c56ce01","size":5,"soft_fail":false,"target_va":5402945568,"va":5372645823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t30304341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e85568ce01","size":5,"soft_fail":false,"target_va":5402950192,"va":5372645846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4760319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e8ffa24800","size":5,"soft_fail":false,"target_va":5377406304,"va":5372645980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4794261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372645424,"raw_hex":"e895274900","size":5,"soft_fail":false,"target_va":5377439776,"va":5372645510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":4752,"parent_record_offset":4608,"ancestor_record_offset":4608,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":4780,"parent_record_offset":4608,"ancestor_record_offset":4608,"inlinee":7550,"source_line":1231,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":4872,"parent_record_offset":4608,"ancestor_record_offset":4608,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":4608,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5304,"parent_record_offset":5280,"ancestor_record_offset":4608,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":4608,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":4608,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5636,"parent_record_offset":5580,"ancestor_record_offset":4608,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5608,"parent_record_offset":5580,"ancestor_record_offset":4608,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":4608,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":4608,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5372,"parent_record_offset":5200,"ancestor_record_offset":4608,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":4608,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":251,"record_offset":5144,"parent_record_offset":5092,"ancestor_record_offset":4608,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":251,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":4608,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":251,"record_offset":7176,"parent_record_offset":7140,"ancestor_record_offset":4608,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":4608,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":8660,"parent_record_offset":4912,"ancestor_record_offset":4608,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":4608,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":5528,"parent_record_offset":5092,"ancestor_record_offset":4608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":4608,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":7804,"parent_record_offset":4912,"ancestor_record_offset":4608,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":6752,"parent_record_offset":4912,"ancestor_record_offset":4608,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":251,"record_offset":5496,"parent_record_offset":5412,"ancestor_record_offset":4608,"inlinee":5032,"source_line":499,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":4608,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":4608,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5056,"parent_record_offset":4912,"ancestor_record_offset":4608,"inlinee":10619,"source_line":2217,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5092,"parent_record_offset":5056,"ancestor_record_offset":4608,"inlinee":10184,"source_line":2185,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5412,"parent_record_offset":5092,"ancestor_record_offset":4608,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":7288,"parent_record_offset":7260,"ancestor_record_offset":4608,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":7344,"parent_record_offset":7316,"ancestor_record_offset":4608,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":7316,"parent_record_offset":7288,"ancestor_record_offset":4608,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":251,"record_offset":7140,"parent_record_offset":4912,"ancestor_record_offset":4608,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":5028,"parent_record_offset":4912,"ancestor_record_offset":4608,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":7260,"parent_record_offset":7232,"ancestor_record_offset":4608,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":7232,"parent_record_offset":7204,"ancestor_record_offset":4608,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":7204,"parent_record_offset":7176,"ancestor_record_offset":4608,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":4812,"parent_record_offset":4780,"ancestor_record_offset":4608,"inlinee":13693,"source_line":24,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":4912,"parent_record_offset":4872,"ancestor_record_offset":4608,"inlinee":13689,"source_line":25,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":7836,"parent_record_offset":7804,"ancestor_record_offset":4608,"inlinee":13691,"source_line":35,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":8596,"parent_record_offset":8564,"ancestor_record_offset":4608,"inlinee":13692,"source_line":40,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x357D
// 首条位置: {"module":251,"inlinee":13693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":4812,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3579
// 首条位置: {"module":251,"inlinee":13689}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":6724,"parent_record_offset":4912,"ancestor_record_offset":4912,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":8660,"parent_record_offset":4912,"ancestor_record_offset":4912,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":6780,"parent_record_offset":4912,"ancestor_record_offset":4912,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":7804,"parent_record_offset":4912,"ancestor_record_offset":4912,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":6752,"parent_record_offset":4912,"ancestor_record_offset":4912,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5056,"parent_record_offset":4912,"ancestor_record_offset":4912,"inlinee":10619,"source_line":2217,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":251,"record_offset":7140,"parent_record_offset":4912,"ancestor_record_offset":4912,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":5028,"parent_record_offset":4912,"ancestor_record_offset":4912,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":4912,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5304,"parent_record_offset":5280,"ancestor_record_offset":4912,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":4912,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5256,"parent_record_offset":5228,"ancestor_record_offset":4912,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":5636,"parent_record_offset":5580,"ancestor_record_offset":4912,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5608,"parent_record_offset":5580,"ancestor_record_offset":4912,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":4912,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":4912,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5372,"parent_record_offset":5200,"ancestor_record_offset":4912,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":4912,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":251,"record_offset":5144,"parent_record_offset":5092,"ancestor_record_offset":4912,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":251,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":4912,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":251,"record_offset":7176,"parent_record_offset":7140,"ancestor_record_offset":4912,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":4912,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":5528,"parent_record_offset":5092,"ancestor_record_offset":4912,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":4912,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":251,"record_offset":5496,"parent_record_offset":5412,"ancestor_record_offset":4912,"inlinee":5032,"source_line":499,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":4912,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":4912,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5092,"parent_record_offset":5056,"ancestor_record_offset":4912,"inlinee":10184,"source_line":2185,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":251,"record_offset":5412,"parent_record_offset":5092,"ancestor_record_offset":4912,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":7288,"parent_record_offset":7260,"ancestor_record_offset":4912,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":7344,"parent_record_offset":7316,"ancestor_record_offset":4912,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":7316,"parent_record_offset":7288,"ancestor_record_offset":4912,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":7260,"parent_record_offset":7232,"ancestor_record_offset":4912,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":7232,"parent_record_offset":7204,"ancestor_record_offset":4912,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":7204,"parent_record_offset":7176,"ancestor_record_offset":4912,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":7836,"parent_record_offset":7804,"ancestor_record_offset":4912,"inlinee":13691,"source_line":35,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":8596,"parent_record_offset":8564,"ancestor_record_offset":4912,"inlinee":13692,"source_line":40,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x357B
// 首条位置: {"module":251,"inlinee":13691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":7836,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x357C
// 首条位置: {"module":251,"inlinee":13692}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":251,"record_offset":8624,"parent_record_offset":8596,"ancestor_record_offset":8596,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":251,"record_offset":9168,"rva":3937968,"va":5372647088}
// 编译实例 RVAs: 0x3c16b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b29a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1109443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e83d12efff","size":5,"soft_fail":false,"target_va":5371537824,"va":5372647262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t102170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e81a8f0100","size":5,"soft_fail":false,"target_va":5372749696,"va":5372647521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t102067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e8b38e0100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372647640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30330427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e83bcece01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372647712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4882469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e825804a00","size":5,"soft_fail":false,"target_va":5377529888,"va":5372647414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4716893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e85df94700","size":5,"soft_fail":false,"target_va":5377364336,"va":5372647438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30331149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e80dd1ce01","size":5,"soft_fail":false,"target_va":5402978880,"va":5372647726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::store_parameters
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1404667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e8fb6e1500","size":5,"soft_fail":false,"target_va":5374051888,"va":5372647216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4792583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647088,"raw_hex":"e807214900","size":5,"soft_fail":false,"target_va":5377439776,"va":5372647188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9312,"parent_record_offset":9168,"ancestor_record_offset":9168,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9340,"parent_record_offset":9168,"ancestor_record_offset":9168,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":9368,"parent_record_offset":9168,"ancestor_record_offset":9168,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":9168,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10228,"parent_record_offset":10204,"ancestor_record_offset":9168,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":9168,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":9168,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10664,"parent_record_offset":10600,"ancestor_record_offset":9168,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":251,"record_offset":9604,"parent_record_offset":9576,"ancestor_record_offset":9168,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":251,"record_offset":9652,"parent_record_offset":9628,"ancestor_record_offset":9168,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10632,"parent_record_offset":10600,"ancestor_record_offset":9168,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10152,"parent_record_offset":10124,"ancestor_record_offset":9168,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10124,"parent_record_offset":10096,"ancestor_record_offset":9168,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10300,"parent_record_offset":10124,"ancestor_record_offset":9168,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10600,"parent_record_offset":10568,"ancestor_record_offset":9168,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":10060,"parent_record_offset":10032,"ancestor_record_offset":9168,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":10032,"parent_record_offset":10004,"ancestor_record_offset":9168,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":9976,"parent_record_offset":9948,"ancestor_record_offset":9168,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":10004,"parent_record_offset":9976,"ancestor_record_offset":9168,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":9948,"parent_record_offset":9920,"ancestor_record_offset":9168,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":251,"record_offset":9920,"parent_record_offset":9400,"ancestor_record_offset":9168,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":251,"record_offset":10096,"parent_record_offset":10060,"ancestor_record_offset":9168,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":251,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":9168,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9788,"parent_record_offset":9400,"ancestor_record_offset":9168,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":9168,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9848,"parent_record_offset":9820,"ancestor_record_offset":9168,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":251,"record_offset":10336,"parent_record_offset":10060,"ancestor_record_offset":9168,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":9688,"parent_record_offset":9576,"ancestor_record_offset":9168,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":10256,"parent_record_offset":10228,"ancestor_record_offset":9168,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":9544,"parent_record_offset":9400,"ancestor_record_offset":9168,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":9752,"parent_record_offset":9400,"ancestor_record_offset":9168,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":10748,"parent_record_offset":9400,"ancestor_record_offset":9168,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::history_chain::HistoryEditChain::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":251,"record_offset":9516,"parent_record_offset":9400,"ancestor_record_offset":9168,"inlinee":13697,"source_line":182,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":251,"record_offset":9884,"parent_record_offset":9820,"ancestor_record_offset":9168,"inlinee":4765,"source_line":188,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":9488,"parent_record_offset":9400,"ancestor_record_offset":9168,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":9400,"parent_record_offset":9368,"ancestor_record_offset":9168,"inlinee":13695,"source_line":59,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":9576,"parent_record_offset":9544,"ancestor_record_offset":9168,"inlinee":13699,"source_line":74,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":9820,"parent_record_offset":9788,"ancestor_record_offset":9168,"inlinee":13700,"source_line":78,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x357F
// 首条位置: {"module":251,"inlinee":13695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":251,"record_offset":9920,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9788,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":10416,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":9544,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":9752,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":10748,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":251,"record_offset":9516,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":13697,"source_line":182,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":9488,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":9400,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10228,"parent_record_offset":10204,"ancestor_record_offset":9400,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":9400,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":9400,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":10664,"parent_record_offset":10600,"ancestor_record_offset":9400,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":251,"record_offset":9604,"parent_record_offset":9576,"ancestor_record_offset":9400,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":251,"record_offset":9652,"parent_record_offset":9628,"ancestor_record_offset":9400,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10632,"parent_record_offset":10600,"ancestor_record_offset":9400,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10152,"parent_record_offset":10124,"ancestor_record_offset":9400,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10124,"parent_record_offset":10096,"ancestor_record_offset":9400,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10300,"parent_record_offset":10124,"ancestor_record_offset":9400,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":10600,"parent_record_offset":10568,"ancestor_record_offset":9400,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":10060,"parent_record_offset":10032,"ancestor_record_offset":9400,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":10032,"parent_record_offset":10004,"ancestor_record_offset":9400,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":9976,"parent_record_offset":9948,"ancestor_record_offset":9400,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":10004,"parent_record_offset":9976,"ancestor_record_offset":9400,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":251,"record_offset":9948,"parent_record_offset":9920,"ancestor_record_offset":9400,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":251,"record_offset":10096,"parent_record_offset":10060,"ancestor_record_offset":9400,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":251,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":9400,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":9400,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9848,"parent_record_offset":9820,"ancestor_record_offset":9400,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":251,"record_offset":10336,"parent_record_offset":10060,"ancestor_record_offset":9400,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":9688,"parent_record_offset":9576,"ancestor_record_offset":9400,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":10256,"parent_record_offset":10228,"ancestor_record_offset":9400,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":251,"record_offset":9884,"parent_record_offset":9820,"ancestor_record_offset":9400,"inlinee":4765,"source_line":188,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":9576,"parent_record_offset":9544,"ancestor_record_offset":9400,"inlinee":13699,"source_line":74,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":251,"record_offset":9820,"parent_record_offset":9788,"ancestor_record_offset":9400,"inlinee":13700,"source_line":78,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3583
// 首条位置: {"module":251,"inlinee":13699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":251,"record_offset":9604,"parent_record_offset":9576,"ancestor_record_offset":9576,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":9688,"parent_record_offset":9576,"ancestor_record_offset":9576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":251,"record_offset":9652,"parent_record_offset":9628,"ancestor_record_offset":9576,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":9576,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3584
// 首条位置: {"module":251,"inlinee":13700}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":9848,"parent_record_offset":9820,"ancestor_record_offset":9820,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":251,"record_offset":9884,"parent_record_offset":9820,"ancestor_record_offset":9820,"inlinee":4765,"source_line":188,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_settings_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":251,"record_offset":11088,"rva":3938768,"va":5372647888}
// 编译实例 RVAs: 0x3c19d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-334261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e84be6faff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372648464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t101600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e8e08c0100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372648107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2523079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e8c77f2600","size":5,"soft_fail":false,"target_va":5375171056,"va":5372647972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h8068e61a17c505da (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30300394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e8ea58ce01","size":5,"soft_fail":false,"target_va":5402948592,"va":5372648193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30330591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e8dfcece01","size":5,"soft_fail":false,"target_va":5402978880,"va":5372648284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3505973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e8357f3500","size":5,"soft_fail":false,"target_va":5376153904,"va":5372647926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4747418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e89a704800","size":5,"soft_fail":false,"target_va":5377395552,"va":5372648129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4791811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e8031e4900","size":5,"soft_fail":false,"target_va":5377439776,"va":5372647960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4789291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e82b144900","size":5,"soft_fail":false,"target_va":5377437280,"va":5372647984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4827102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372647888,"raw_hex":"e8dea74900","size":5,"soft_fail":false,"target_va":5377475184,"va":5372648077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":251,"record_offset":12548,"parent_record_offset":11088,"ancestor_record_offset":11088,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":11456,"parent_record_offset":11088,"ancestor_record_offset":11088,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":13004,"parent_record_offset":11088,"ancestor_record_offset":11088,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":12656,"parent_record_offset":11088,"ancestor_record_offset":11088,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":251,"record_offset":11520,"parent_record_offset":11088,"ancestor_record_offset":11088,"inlinee":4754,"source_line":2066,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":11492,"parent_record_offset":11088,"ancestor_record_offset":11088,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":11316,"parent_record_offset":11088,"ancestor_record_offset":11088,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::settings_file_path
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/app_config.rs
//     evidence={"module":251,"record_offset":11260,"parent_record_offset":11088,"ancestor_record_offset":11088,"inlinee":12916,"source_line":149,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":12184,"parent_record_offset":12160,"ancestor_record_offset":11088,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":12160,"parent_record_offset":12104,"ancestor_record_offset":11088,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":12132,"parent_record_offset":12104,"ancestor_record_offset":11088,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":11088,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":251,"record_offset":12616,"parent_record_offset":12580,"ancestor_record_offset":11088,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":251,"record_offset":12580,"parent_record_offset":12548,"ancestor_record_offset":11088,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":251,"record_offset":11720,"parent_record_offset":11520,"ancestor_record_offset":11088,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":11944,"parent_record_offset":11520,"ancestor_record_offset":11088,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":11364,"parent_record_offset":11340,"ancestor_record_offset":11088,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":11412,"parent_record_offset":11388,"ancestor_record_offset":11088,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":11388,"parent_record_offset":11364,"ancestor_record_offset":11088,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":11284,"parent_record_offset":11260,"ancestor_record_offset":11088,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":11088,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":11576,"parent_record_offset":11552,"ancestor_record_offset":11088,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":11552,"parent_record_offset":11520,"ancestor_record_offset":11088,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_profile_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":251,"record_offset":13608,"rva":3939440,"va":5372648560}
// 编译实例 RVAs: 0x3c1c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-334901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e8cbe3faff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372649104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t100955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e85b8a0100","size":5,"soft_fail":false,"target_va":5372749712,"va":5372648752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2522434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e8427d2600","size":5,"soft_fail":false,"target_va":5375171056,"va":5372648617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h8068e61a17c505da (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30299749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e86556ce01","size":5,"soft_fail":false,"target_va":5402948592,"va":5372648838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30329946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e85accce01","size":5,"soft_fail":false,"target_va":5402978880,"va":5372648929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_profile::profile_file_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2043214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e84e2d1f00","size":5,"soft_fail":false,"target_va":5374691824,"va":5372648605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4746773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e8156e4800","size":5,"soft_fail":false,"target_va":5377395552,"va":5372648774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4791106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e8421b4900","size":5,"soft_fail":false,"target_va":5377439776,"va":5372648665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4788646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e8a6114900","size":5,"soft_fail":false,"target_va":5377437280,"va":5372648629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4826457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372648560,"raw_hex":"e859a54900","size":5,"soft_fail":false,"target_va":5377475184,"va":5372648722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":251,"record_offset":15008,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":251,"record_offset":13916,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":15464,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":251,"record_offset":15116,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":251,"record_offset":13980,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":4754,"source_line":2066,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":13952,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":13776,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":14644,"parent_record_offset":14620,"ancestor_record_offset":13608,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":251,"record_offset":14620,"parent_record_offset":14564,"ancestor_record_offset":13608,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":14592,"parent_record_offset":14564,"ancestor_record_offset":13608,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":251,"record_offset":14564,"parent_record_offset":14536,"ancestor_record_offset":13608,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":251,"record_offset":15076,"parent_record_offset":15040,"ancestor_record_offset":13608,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":251,"record_offset":15040,"parent_record_offset":15008,"ancestor_record_offset":13608,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":251,"record_offset":14180,"parent_record_offset":13980,"ancestor_record_offset":13608,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":251,"record_offset":14404,"parent_record_offset":13980,"ancestor_record_offset":13608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":13608,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":13608,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":251,"record_offset":13848,"parent_record_offset":13824,"ancestor_record_offset":13608,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":13608,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":13608,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00251.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":251,"record_offset":14012,"parent_record_offset":13980,"ancestor_record_offset":13608,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00251.jsonl.gz"}
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
// 完整结构及原始类型记录: panic_dump.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: panic_dump.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: panic_dump.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs
// 信息 ID: b029e0ae58691c6fedb5c07c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278d98 -> 0x3c1030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
// 信息 ID: 13c7bb76dfe70d4015d66459
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=251, inline_record=4812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
// 信息 ID: eca2c96b2f4f4dfac968a53e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=251, inline_record=4912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$0
// 信息 ID: 80e24f8d15097d180d5e58db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=251, inline_record=7836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
// 信息 ID: 66860bfe78fffff3cf452a31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=251, inline_record=8596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_log_file
// 信息 ID: 49c53ea6f22a5e0e0c08b91a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278e45 -> 0x3c0e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_profile_file
// 信息 ID: 72809a06ea46e25c983e54ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278f9f -> 0x3c1c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save
// 信息 ID: 204c3bf73d66fdeb39370780
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278ca7 -> 0x3c16b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
// 信息 ID: 9ba777568841478e749add1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=251, inline_record=9400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
// 信息 ID: 01830dd922b04ec3a621f5de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=251, inline_record=9576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
// 信息 ID: 468ba6d6c4aecbc7c0618c4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=251, inline_record=9820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_settings_file
// 信息 ID: f8135a04ef64173e3349d3bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278ef2 -> 0x3c19d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
