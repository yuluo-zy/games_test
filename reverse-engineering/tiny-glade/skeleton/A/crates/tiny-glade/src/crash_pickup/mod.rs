// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/crash_pickup/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: aa37d61f3104ab6776320fbc70138c286f9501f28d0b3582dc3fe5dfe1abb98f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 10, 145, 236, 244
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use minidumper; // namespace candidate
//   package identities: []
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$4::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":244,"record_offset":17804,"rva":3887568,"va":5372596688}
// 编译实例 RVAs: 0x3b51d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4983677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372596688,"raw_hex":"e87d0b4c00","size":5,"soft_fail":false,"target_va":5377580528,"va":5372596846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] _$LT$crash_handler..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t1168490","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372596688,"raw_hex":"e96ad41100","size":5,"soft_fail":false,"target_va":5373765360,"va":5372596865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] _$LT$minidumper..errors..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t1166687","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372596688,"raw_hex":"e95fcd1100","size":5,"soft_fail":false,"target_va":5373763440,"va":5372596748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] _$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t4815801","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5372596688,"raw_hex":"e9b97b4900","size":5,"soft_fail":false,"target_va":5377412544,"va":5372596738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Formatter::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":244,"record_offset":17928,"parent_record_offset":17804,"ancestor_record_offset":17804,"inlinee":5667,"source_line":1619,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$8::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":145,"record_offset":17748,"rva":2595360,"va":5371304480}
// 编译实例 RVAs: 0x279a20, 0x39b770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t6281316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371304480,"raw_hex":"e864d85f00","size":5,"soft_fail":false,"target_va":5377585888,"va":5371304567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x34A1
// 首条位置: {"module":236,"inlinee":13473}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":236,"record_offset":14052,"parent_record_offset":14024,"ancestor_record_offset":14024,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":14076,"parent_record_offset":14052,"ancestor_record_offset":14024,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":236,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":14024,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::ClientHandle::send_raw_message
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14E2
// 首条位置: {"module":244,"inlinee":5346}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":244,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":11252,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":11688,"parent_record_offset":11252,"ancestor_record_offset":11252,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::send_message
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":244,"record_offset":11308,"parent_record_offset":11252,"ancestor_record_offset":11252,"inlinee":5328,"source_line":166,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":11252,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":11444,"parent_record_offset":11388,"ancestor_record_offset":11252,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":11416,"parent_record_offset":11388,"ancestor_record_offset":11252,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":11388,"parent_record_offset":11364,"ancestor_record_offset":11252,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":11340,"parent_record_offset":11308,"ancestor_record_offset":11252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::ClientHandle::send_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":244,"record_offset":10316,"rva":3884688,"va":5372593808}
// 编译实例 RVAs: 0x3b4690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ddab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-879689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372593808,"raw_hex":"e8b793f2ff","size":5,"soft_fail":false,"target_va":5371714224,"va":5372593908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-273866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372593808,"raw_hex":"e836d2fbff","size":5,"soft_fail":false,"target_va":5372320512,"va":5372594373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403b44d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372593808,"raw_hex":"e845fdffff","size":5,"soft_fail":false,"target_va":5372593360,"va":5372594054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t155836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372593808,"raw_hex":"e8bc600200","size":5,"soft_fail":false,"target_va":5372749696,"va":5372593855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t155700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372593808,"raw_hex":"e834600200","size":5,"soft_fail":false,"target_va":5372749712,"va":5372594007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30384058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372593808,"raw_hex":"e8ba9fcf01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372594081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30385964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372593808,"raw_hex":"e82ca7cf01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372594159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::client::Client::send_message_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1170620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372593808,"raw_hex":"e8bcdc1100","size":5,"soft_fail":false,"target_va":5373764608,"va":5372593983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":11212,"parent_record_offset":10316,"ancestor_record_offset":10316,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":244,"record_offset":10460,"parent_record_offset":10316,"ancestor_record_offset":10316,"inlinee":10184,"source_line":2185,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::ClientHandle::send_raw_message
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":244,"record_offset":11252,"parent_record_offset":10316,"ancestor_record_offset":10316,"inlinee":5346,"source_line":38,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":10640,"parent_record_offset":10616,"ancestor_record_offset":10316,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":10316,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":10988,"parent_record_offset":10964,"ancestor_record_offset":10316,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":10616,"parent_record_offset":10588,"ancestor_record_offset":10316,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":10964,"parent_record_offset":10908,"ancestor_record_offset":10316,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":10936,"parent_record_offset":10908,"ancestor_record_offset":10316,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":10588,"parent_record_offset":10560,"ancestor_record_offset":10316,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":10560,"parent_record_offset":10532,"ancestor_record_offset":10316,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":10732,"parent_record_offset":10560,"ancestor_record_offset":10316,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":10908,"parent_record_offset":10880,"ancestor_record_offset":10316,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":244,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":10316,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":244,"record_offset":10504,"parent_record_offset":10460,"ancestor_record_offset":10316,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":244,"record_offset":10532,"parent_record_offset":10504,"ancestor_record_offset":10316,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":10856,"parent_record_offset":10460,"ancestor_record_offset":10316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":10688,"parent_record_offset":10664,"ancestor_record_offset":10316,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":11688,"parent_record_offset":11252,"ancestor_record_offset":10316,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::client::Client::send_message
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":244,"record_offset":11308,"parent_record_offset":11252,"ancestor_record_offset":10316,"inlinee":5328,"source_line":166,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":244,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":10316,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":244,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":10316,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":244,"record_offset":10772,"parent_record_offset":10460,"ancestor_record_offset":10316,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3499
// 首条位置: {"module":236,"inlinee":13465}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":5124,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":5092,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":4952,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":4916,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":5340,"parent_record_offset":5284,"ancestor_record_offset":4916,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":4916,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":4916,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":4916,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4916,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4916,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4916,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":4916,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3497
// 首条位置: {"module":236,"inlinee":13463}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tiny_glade::crash_pickup::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":236,"record_offset":4916,"parent_record_offset":4888,"ancestor_record_offset":4888,"inlinee":13465,"source_line":54,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":4888,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":5340,"parent_record_offset":5284,"ancestor_record_offset":4888,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":4888,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":4888,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":5124,"parent_record_offset":4916,"ancestor_record_offset":4888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4888,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4888,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":236,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4888,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":5092,"parent_record_offset":4916,"ancestor_record_offset":4888,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":4888,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":236,"record_offset":4952,"parent_record_offset":4916,"ancestor_record_offset":4888,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::run_server
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x349B
// 首条位置: {"module":236,"inlinee":13467}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":8808,"parent_record_offset":8776,"ancestor_record_offset":8776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":8776,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":236,"record_offset":8936,"parent_record_offset":8880,"ancestor_record_offset":8776,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":8908,"parent_record_offset":8880,"ancestor_record_offset":8776,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":236,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":8776,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":236,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":8776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::CrashReporter::spawn_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":244,"record_offset":11728,"rva":3885280,"va":5372594400}
// 编译实例 RVAs: 0x3b48e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14036f640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-284651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e815a8fbff","size":5,"soft_fail":false,"target_va":5372311104,"va":5372595750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-280814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e812b7fbff","size":5,"soft_fail":false,"target_va":5372314416,"va":5372595225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140393d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-136083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e86decfdff","size":5,"soft_fail":false,"target_va":5372460432,"va":5372596510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t154588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8dc5b0200","size":5,"soft_fail":false,"target_va":5372749696,"va":5372595103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t155017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8895d0200","size":5,"soft_fail":false,"target_va":5372749712,"va":5372594690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2576589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8cd502700","size":5,"soft_fail":false,"target_va":5375171056,"va":5372594462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140749d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3756889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e859533900","size":5,"soft_fail":false,"target_va":5376351488,"va":5372594594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4810262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e816664900","size":5,"soft_fail":false,"target_va":5377405008,"va":5372594741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30998369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e861ffd801","size":5,"soft_fail":false,"target_va":5403593360,"va":5372594986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30382326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8f698cf01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372595813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-945290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e87693f1ff","size":5,"soft_fail":false,"target_va":5371651312,"va":5372596597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30384325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8c5a0cf01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372595798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_exe
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4807626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8ca5b4900","size":5,"soft_fail":false,"target_va":5377402512,"va":5372594881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::temp_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4807254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8565a4900","size":5,"soft_fail":false,"target_va":5377401712,"va":5372594453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4845111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e837ee4900","size":5,"soft_fail":false,"target_va":5377439776,"va":5372594660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::process::Child::id
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4846352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e810f34900","size":5,"soft_fail":false,"target_va":5377441760,"va":5372595403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::process::Command::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4846091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e80bf24900","size":5,"soft_fail":false,"target_va":5377441312,"va":5372595216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::process::Command::arg
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4889997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e88d9d4a00","size":5,"soft_fail":false,"target_va":5377485040,"va":5372595038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::process::Command::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4889796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8c49c4a00","size":5,"soft_fail":false,"target_va":5377484768,"va":5372594967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::client::start
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2350931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e8ad20dcff","size":5,"soft_fail":false,"target_va":5370244528,"va":5372595454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3757611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372594400,"raw_hex":"e82b563900","size":5,"soft_fail":false,"target_va":5376352096,"va":5372594480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":244,"record_offset":12040,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":244,"record_offset":13356,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":12824,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":14080,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":15924,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":13384,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":244,"record_offset":12388,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":244,"record_offset":13264,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":10965,"source_line":2109,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":244,"record_offset":11900,"parent_record_offset":11728,"ancestor_record_offset":11728,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":15324,"parent_record_offset":15300,"ancestor_record_offset":11728,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":15348,"parent_record_offset":15324,"ancestor_record_offset":11728,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":11728,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16216,"parent_record_offset":16180,"ancestor_record_offset":11728,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":15300,"parent_record_offset":15272,"ancestor_record_offset":11728,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":12312,"parent_record_offset":12256,"ancestor_record_offset":11728,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":244,"record_offset":16180,"parent_record_offset":16144,"ancestor_record_offset":11728,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":12284,"parent_record_offset":12256,"ancestor_record_offset":11728,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":15272,"parent_record_offset":15244,"ancestor_record_offset":11728,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":15244,"parent_record_offset":15216,"ancestor_record_offset":11728,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":15420,"parent_record_offset":15244,"ancestor_record_offset":11728,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":12256,"parent_record_offset":12228,"ancestor_record_offset":11728,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15180,"parent_record_offset":15148,"ancestor_record_offset":11728,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15148,"parent_record_offset":15116,"ancestor_record_offset":11728,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15084,"parent_record_offset":15052,"ancestor_record_offset":11728,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15116,"parent_record_offset":15084,"ancestor_record_offset":11728,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15052,"parent_record_offset":15020,"ancestor_record_offset":11728,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":244,"record_offset":15020,"parent_record_offset":14988,"ancestor_record_offset":11728,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":244,"record_offset":12088,"parent_record_offset":12040,"ancestor_record_offset":11728,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":244,"record_offset":14960,"parent_record_offset":14124,"ancestor_record_offset":11728,"inlinee":13599,"source_line":2584,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":244,"record_offset":14988,"parent_record_offset":14960,"ancestor_record_offset":11728,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":244,"record_offset":16144,"parent_record_offset":16104,"ancestor_record_offset":11728,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":244,"record_offset":16400,"parent_record_offset":16376,"ancestor_record_offset":11728,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":244,"record_offset":15216,"parent_record_offset":15180,"ancestor_record_offset":11728,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":244,"record_offset":12116,"parent_record_offset":12040,"ancestor_record_offset":11728,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":244,"record_offset":15480,"parent_record_offset":15456,"ancestor_record_offset":11728,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":244,"record_offset":15456,"parent_record_offset":15180,"ancestor_record_offset":11728,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=66
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":12180,"parent_record_offset":12040,"ancestor_record_offset":11728,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":15376,"parent_record_offset":15348,"ancestor_record_offset":11728,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":12144,"parent_record_offset":12040,"ancestor_record_offset":11728,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":16056,"parent_record_offset":16000,"ancestor_record_offset":11728,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":13320,"parent_record_offset":13292,"ancestor_record_offset":11728,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16436,"parent_record_offset":16400,"ancestor_record_offset":11728,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":11728,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16492,"parent_record_offset":16400,"ancestor_record_offset":11728,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":244,"record_offset":13292,"parent_record_offset":13264,"ancestor_record_offset":11728,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":244,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":11728,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":244,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":11728,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":244,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":11728,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":244,"record_offset":11924,"parent_record_offset":11900,"ancestor_record_offset":11728,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$62::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":244,"record_offset":14216,"parent_record_offset":14184,"ancestor_record_offset":11728,"inlinee":9902,"source_line":1948,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::process::Command::arg
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\process.rs
//     evidence={"module":244,"record_offset":14876,"parent_record_offset":14124,"ancestor_record_offset":11728,"inlinee":9912,"source_line":691,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::process::Command::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\process.rs
//     evidence={"module":244,"record_offset":14184,"parent_record_offset":14124,"ancestor_record_offset":11728,"inlinee":9913,"source_line":629,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":244,"record_offset":14124,"parent_record_offset":14080,"ancestor_record_offset":11728,"inlinee":13597,"source_line":89,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":244,"record_offset":16000,"parent_record_offset":15960,"ancestor_record_offset":11728,"inlinee":13600,"source_line":98,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":244,"record_offset":16104,"parent_record_offset":16056,"ancestor_record_offset":11728,"inlinee":13602,"source_line":105,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x351D
// 首条位置: {"module":244,"inlinee":13597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":244,"record_offset":14960,"parent_record_offset":14124,"ancestor_record_offset":14124,"inlinee":13599,"source_line":2584,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] std::process::Command::arg
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\process.rs
//     evidence={"module":244,"record_offset":14876,"parent_record_offset":14124,"ancestor_record_offset":14124,"inlinee":9912,"source_line":691,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] std::process::Command::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\process.rs
//     evidence={"module":244,"record_offset":14184,"parent_record_offset":14124,"ancestor_record_offset":14124,"inlinee":9913,"source_line":629,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":15324,"parent_record_offset":15300,"ancestor_record_offset":14124,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":15348,"parent_record_offset":15324,"ancestor_record_offset":14124,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":14512,"parent_record_offset":14488,"ancestor_record_offset":14124,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":15300,"parent_record_offset":15272,"ancestor_record_offset":14124,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":14488,"parent_record_offset":14432,"ancestor_record_offset":14124,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":14460,"parent_record_offset":14432,"ancestor_record_offset":14124,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":15272,"parent_record_offset":15244,"ancestor_record_offset":14124,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":15244,"parent_record_offset":15216,"ancestor_record_offset":14124,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":15420,"parent_record_offset":15244,"ancestor_record_offset":14124,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":244,"record_offset":14432,"parent_record_offset":14404,"ancestor_record_offset":14124,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15180,"parent_record_offset":15148,"ancestor_record_offset":14124,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15148,"parent_record_offset":15116,"ancestor_record_offset":14124,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15084,"parent_record_offset":15052,"ancestor_record_offset":14124,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15116,"parent_record_offset":15084,"ancestor_record_offset":14124,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":244,"record_offset":15052,"parent_record_offset":15020,"ancestor_record_offset":14124,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":244,"record_offset":15020,"parent_record_offset":14988,"ancestor_record_offset":14124,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":244,"record_offset":14988,"parent_record_offset":14960,"ancestor_record_offset":14124,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":244,"record_offset":15216,"parent_record_offset":15180,"ancestor_record_offset":14124,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":244,"record_offset":15480,"parent_record_offset":15456,"ancestor_record_offset":14124,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":244,"record_offset":15456,"parent_record_offset":15180,"ancestor_record_offset":14124,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":14272,"parent_record_offset":14184,"ancestor_record_offset":14124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":15376,"parent_record_offset":15348,"ancestor_record_offset":14124,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":244,"record_offset":14240,"parent_record_offset":14216,"ancestor_record_offset":14124,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$62::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":244,"record_offset":14216,"parent_record_offset":14184,"ancestor_record_offset":14124,"inlinee":9902,"source_line":1948,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3520
// 首条位置: {"module":244,"inlinee":13600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":244,"record_offset":16056,"parent_record_offset":16000,"ancestor_record_offset":16000,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":16000,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":16000,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16216,"parent_record_offset":16180,"ancestor_record_offset":16000,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16248,"parent_record_offset":16216,"ancestor_record_offset":16000,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":244,"record_offset":16180,"parent_record_offset":16144,"ancestor_record_offset":16000,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":244,"record_offset":16144,"parent_record_offset":16104,"ancestor_record_offset":16000,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":244,"record_offset":16400,"parent_record_offset":16376,"ancestor_record_offset":16000,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":16376,"parent_record_offset":16104,"ancestor_record_offset":16000,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":16324,"parent_record_offset":16296,"ancestor_record_offset":16000,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16436,"parent_record_offset":16400,"ancestor_record_offset":16000,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":16000,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16492,"parent_record_offset":16400,"ancestor_record_offset":16000,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$2::spawn_server::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":244,"record_offset":16104,"parent_record_offset":16056,"ancestor_record_offset":16000,"inlinee":13602,"source_line":105,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3522
// 首条位置: {"module":244,"inlinee":13602}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":244,"record_offset":16144,"parent_record_offset":16104,"ancestor_record_offset":16104,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":16376,"parent_record_offset":16104,"ancestor_record_offset":16104,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":16104,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":16104,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16216,"parent_record_offset":16180,"ancestor_record_offset":16104,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":244,"record_offset":16248,"parent_record_offset":16216,"ancestor_record_offset":16104,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":244,"record_offset":16180,"parent_record_offset":16144,"ancestor_record_offset":16104,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":244,"record_offset":16400,"parent_record_offset":16376,"ancestor_record_offset":16104,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":244,"record_offset":16324,"parent_record_offset":16296,"ancestor_record_offset":16104,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16436,"parent_record_offset":16400,"ancestor_record_offset":16104,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":16104,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00244.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":244,"record_offset":16492,"parent_record_offset":16400,"ancestor_record_offset":16104,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00244.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::ClientHandle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::ClientHandle>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::ClientHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::CrashHandlerHooks
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<tiny_glade::crash_pickup::CrashHandlerHooks,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["tiny_glade::crash_pickup::CrashHandlerHooks", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<tiny_glade::crash_pickup::CrashHandlerHooks>", "constructor": "alloc::sync::ArcInner", "arguments": ["tiny_glade::crash_pickup::CrashHandlerHooks"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<tiny_glade::crash_pickup::CrashHandlerHooks> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<tiny_glade::crash_pickup::CrashHandlerHooks>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::CrashReporter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::Error
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::crash_pickup::Error>", "constructor": "enum2$", "arguments": ["tiny_glade::crash_pickup::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::crash_pickup::Error> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::crash_pickup::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result::expect<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >", "constructor": "core::result::Result::expect", "arguments": ["tuple$<>", "enum2$<tiny_glade::crash_pickup::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<tiny_glade::crash_pickup::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::ClientHandle::send_message
// 信息 ID: b714f88380e02f91f2b750e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0x278994 -> 0x3b4690 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c96df -> 0x3b4690 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c974e -> 0x3b4690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::ClientHandle::send_raw_message
// 信息 ID: 46810646e821048f5a2fc319
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::ClientHandle::send_message; module=244, inline_record=11252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::new
// 信息 ID: 9af244133c36cb263036d356
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=236, inline_record=4888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::run_server
// 信息 ID: 537d649ee2851264f248e774
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=236, inline_record=8776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::CrashReporter::spawn_server
// 信息 ID: cd3a26468bcbdeb5011a6cb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x399d27 -> 0x3b48e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$1::default
// 信息 ID: 1fe01ebb65ef45f7aa6913a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=236, inline_record=4916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$2::spawn_server::closure$0
// 信息 ID: a390a22bbe34f238b738e691
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::impl$2::spawn_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::CrashReporter::spawn_server; module=244, inline_record=14124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1
// 信息 ID: 25b21827fe597eabe4718b39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::impl$2::spawn_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::CrashReporter::spawn_server; module=244, inline_record=16000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$2::spawn_server::closure$1::closure$0
// 信息 ID: f5018c0ee8994c0adacaa241
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::impl$2::spawn_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::CrashReporter::spawn_server; module=244, inline_record=16104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$4::fmt
// 信息 ID: 904d6f38d9282bbf177ee292
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$8::fmt
// 信息 ID: c604eae98ba836049c4df69d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::impl$9::clone
// 信息 ID: f67f142011b16add542f583a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=236, inline_record=14024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
