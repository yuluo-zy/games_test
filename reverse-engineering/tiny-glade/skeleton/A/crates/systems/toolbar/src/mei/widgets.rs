// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/mei/widgets.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0078f23cc31dfd9dcd958734cdcfb6f43ec8f48a97e54788f44304fc4001357a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: widgets.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 15, 31, 58, 148, 187, 220, 318, 319, 1018, 1025, 2421, 2425, 2427, 2428, 2468, 2499, 3457, 3458, 3685, 4244
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::label
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=48
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x440D
// 首条位置: {"module":319,"record_offset":72192,"rva":4470128,"va":5373179248}
// 编译实例 RVAs: 0x443570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179248,"raw_hex":"e80fa1c9ff","size":5,"soft_fail":false,"target_va":5369616080,"va":5373179324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3562873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179248,"raw_hex":"e887a2c9ff","size":5,"soft_fail":false,"target_va":5369616480,"va":5373179348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3561636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179248,"raw_hex":"e85ca7c9ff","size":5,"soft_fail":false,"target_va":5369617648,"va":5373179279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6383862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179248,"raw_hex":"e8f6686100","size":5,"soft_fail":false,"target_va":5379563248,"va":5373179381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6393640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179248,"raw_hex":"e8288f6100","size":5,"soft_fail":false,"target_va":5379573040,"va":5373179395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179248,"raw_hex":"e8bcb7ffff","size":5,"soft_fail":false,"target_va":5373160864,"va":5373179359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":55324,"parent_record_offset":55264,"ancestor_record_offset":55264,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=42
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72344,"parent_record_offset":72192,"ancestor_record_offset":72192,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":15124,"parent_record_offset":14948,"ancestor_record_offset":14948,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=39
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72316,"parent_record_offset":72192,"ancestor_record_offset":72192,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":55452,"parent_record_offset":55428,"ancestor_record_offset":55264,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2468,"record_offset":55428,"parent_record_offset":55372,"ancestor_record_offset":55264,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":55400,"parent_record_offset":55372,"ancestor_record_offset":55264,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2468,"record_offset":55372,"parent_record_offset":55348,"ancestor_record_offset":55264,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2468,"record_offset":55348,"parent_record_offset":55324,"ancestor_record_offset":55264,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=41
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72404,"parent_record_offset":72344,"ancestor_record_offset":72192,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=39
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72376,"parent_record_offset":72344,"ancestor_record_offset":72192,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=43
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72428,"parent_record_offset":72404,"ancestor_record_offset":72192,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=35
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":319,"record_offset":72460,"parent_record_offset":72344,"ancestor_record_offset":72192,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::label::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=41
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43D5
// 首条位置: {"module":319,"inlinee":17365}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::label_with_id
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6D4
// 首条位置: {"module":1018,"inlinee":50900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77300,"parent_record_offset":77276,"ancestor_record_offset":77276,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77356,"parent_record_offset":77300,"ancestor_record_offset":77276,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77328,"parent_record_offset":77300,"ancestor_record_offset":77276,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77380,"parent_record_offset":77356,"ancestor_record_offset":77276,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label_with_id::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":77412,"parent_record_offset":77300,"ancestor_record_offset":77276,"inlinee":50902,"source_line":56,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::label_with_id::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6D6
// 首条位置: {"module":1018,"inlinee":50902}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::menu_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":319,"record_offset":72496,"rva":4470304,"va":5373179424}
// 编译实例 RVAs: 0x443620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3566101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179424,"raw_hex":"e8eb95c9ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373179616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179424,"raw_hex":"e85fa0c9ff","size":5,"soft_fail":false,"target_va":5369616080,"va":5373179500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179424,"raw_hex":"e8d7a1c9ff","size":5,"soft_fail":false,"target_va":5369616480,"va":5373179524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3561812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179424,"raw_hex":"e8aca6c9ff","size":5,"soft_fail":false,"target_va":5369617648,"va":5373179455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6386854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179424,"raw_hex":"e8a6746100","size":5,"soft_fail":false,"target_va":5379566416,"va":5373179557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6393464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179424,"raw_hex":"e8788e6100","size":5,"soft_fail":false,"target_va":5379573040,"va":5373179571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29799075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179424,"raw_hex":"e8a3b2c601","size":5,"soft_fail":false,"target_va":5402978880,"va":5373179800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179424,"raw_hex":"e80cb7ffff","size":5,"soft_fail":false,"target_va":5373160864,"va":5373179535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72660,"parent_record_offset":72496,"ancestor_record_offset":72496,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":319,"record_offset":72808,"parent_record_offset":72496,"ancestor_record_offset":72496,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72632,"parent_record_offset":72496,"ancestor_record_offset":72496,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":319,"record_offset":73644,"parent_record_offset":73620,"ancestor_record_offset":72496,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":319,"record_offset":73112,"parent_record_offset":73088,"ancestor_record_offset":72496,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":319,"record_offset":73196,"parent_record_offset":73172,"ancestor_record_offset":72496,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":319,"record_offset":73136,"parent_record_offset":73112,"ancestor_record_offset":72496,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":319,"record_offset":73308,"parent_record_offset":73284,"ancestor_record_offset":72496,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":319,"record_offset":73860,"parent_record_offset":72836,"ancestor_record_offset":72496,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":319,"record_offset":73556,"parent_record_offset":73532,"ancestor_record_offset":72496,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":319,"record_offset":73448,"parent_record_offset":73424,"ancestor_record_offset":72496,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":319,"record_offset":73620,"parent_record_offset":73596,"ancestor_record_offset":72496,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":72912,"parent_record_offset":72888,"ancestor_record_offset":72496,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":72888,"parent_record_offset":72864,"ancestor_record_offset":72496,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":73596,"parent_record_offset":73376,"ancestor_record_offset":72496,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":72940,"parent_record_offset":72912,"ancestor_record_offset":72496,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":73508,"parent_record_offset":73484,"ancestor_record_offset":72496,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":73484,"parent_record_offset":73376,"ancestor_record_offset":72496,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":73424,"parent_record_offset":73400,"ancestor_record_offset":72496,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":73804,"parent_record_offset":73016,"ancestor_record_offset":72496,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":73400,"parent_record_offset":73376,"ancestor_record_offset":72496,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":72992,"parent_record_offset":72968,"ancestor_record_offset":72496,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":72968,"parent_record_offset":72912,"ancestor_record_offset":72496,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":73016,"parent_record_offset":72992,"ancestor_record_offset":72496,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":73256,"parent_record_offset":73228,"ancestor_record_offset":72496,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":73284,"parent_record_offset":73256,"ancestor_record_offset":72496,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":73344,"parent_record_offset":73228,"ancestor_record_offset":72496,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":73228,"parent_record_offset":73016,"ancestor_record_offset":72496,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":73060,"parent_record_offset":73016,"ancestor_record_offset":72496,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":73376,"parent_record_offset":73016,"ancestor_record_offset":72496,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":73088,"parent_record_offset":73016,"ancestor_record_offset":72496,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":73172,"parent_record_offset":73016,"ancestor_record_offset":72496,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":73712,"parent_record_offset":73016,"ancestor_record_offset":72496,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":319,"record_offset":72864,"parent_record_offset":72836,"ancestor_record_offset":72496,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":73892,"parent_record_offset":72808,"ancestor_record_offset":72496,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72720,"parent_record_offset":72660,"ancestor_record_offset":72496,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72836,"parent_record_offset":72808,"ancestor_record_offset":72496,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":73668,"parent_record_offset":73644,"ancestor_record_offset":72496,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72692,"parent_record_offset":72660,"ancestor_record_offset":72496,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":72744,"parent_record_offset":72720,"ancestor_record_offset":72496,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::menu_button::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":319,"record_offset":72776,"parent_record_offset":72660,"ancestor_record_offset":72496,"inlinee":17367,"source_line":63,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::menu_button::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43D7
// 首条位置: {"module":319,"inlinee":17367}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::menu_button_with_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":319,"record_offset":73928,"rva":4470688,"va":5373179808}
// 编译实例 RVAs: 0x4437a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3566465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179808,"raw_hex":"e87f94c9ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373179980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179808,"raw_hex":"e8f39ec9ff","size":5,"soft_fail":false,"target_va":5369616080,"va":5373179864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179808,"raw_hex":"e86ba0c9ff","size":5,"soft_fail":false,"target_va":5369616480,"va":5373179888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6386490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179808,"raw_hex":"e83a736100","size":5,"soft_fail":false,"target_va":5379566416,"va":5373179921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6393100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179808,"raw_hex":"e80c8d6100","size":5,"soft_fail":false,"target_va":5379573040,"va":5373179935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29798707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179808,"raw_hex":"e833b1c601","size":5,"soft_fail":false,"target_va":5402978880,"va":5373180168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373179808,"raw_hex":"e8a0b5ffff","size":5,"soft_fail":false,"target_va":5373160864,"va":5373179899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":74068,"parent_record_offset":73928,"ancestor_record_offset":73928,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":319,"record_offset":74216,"parent_record_offset":73928,"ancestor_record_offset":73928,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":319,"record_offset":75052,"parent_record_offset":75028,"ancestor_record_offset":73928,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":319,"record_offset":74520,"parent_record_offset":74496,"ancestor_record_offset":73928,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":319,"record_offset":74604,"parent_record_offset":74580,"ancestor_record_offset":73928,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":319,"record_offset":74544,"parent_record_offset":74520,"ancestor_record_offset":73928,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":319,"record_offset":74716,"parent_record_offset":74692,"ancestor_record_offset":73928,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":319,"record_offset":75268,"parent_record_offset":74244,"ancestor_record_offset":73928,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":319,"record_offset":74964,"parent_record_offset":74940,"ancestor_record_offset":73928,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":319,"record_offset":74856,"parent_record_offset":74832,"ancestor_record_offset":73928,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":319,"record_offset":75028,"parent_record_offset":75004,"ancestor_record_offset":73928,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":74320,"parent_record_offset":74296,"ancestor_record_offset":73928,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":74296,"parent_record_offset":74272,"ancestor_record_offset":73928,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":75004,"parent_record_offset":74784,"ancestor_record_offset":73928,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":319,"record_offset":74348,"parent_record_offset":74320,"ancestor_record_offset":73928,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74916,"parent_record_offset":74892,"ancestor_record_offset":73928,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74892,"parent_record_offset":74784,"ancestor_record_offset":73928,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74832,"parent_record_offset":74808,"ancestor_record_offset":73928,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":75212,"parent_record_offset":74424,"ancestor_record_offset":73928,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74808,"parent_record_offset":74784,"ancestor_record_offset":73928,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74400,"parent_record_offset":74376,"ancestor_record_offset":73928,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74376,"parent_record_offset":74320,"ancestor_record_offset":73928,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74424,"parent_record_offset":74400,"ancestor_record_offset":73928,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":74664,"parent_record_offset":74636,"ancestor_record_offset":73928,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":74692,"parent_record_offset":74664,"ancestor_record_offset":73928,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":74752,"parent_record_offset":74636,"ancestor_record_offset":73928,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":319,"record_offset":74636,"parent_record_offset":74424,"ancestor_record_offset":73928,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74468,"parent_record_offset":74424,"ancestor_record_offset":73928,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":319,"record_offset":74784,"parent_record_offset":74424,"ancestor_record_offset":73928,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":74496,"parent_record_offset":74424,"ancestor_record_offset":73928,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":74580,"parent_record_offset":74424,"ancestor_record_offset":73928,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":319,"record_offset":75120,"parent_record_offset":74424,"ancestor_record_offset":73928,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":319,"record_offset":74272,"parent_record_offset":74244,"ancestor_record_offset":73928,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":75300,"parent_record_offset":74216,"ancestor_record_offset":73928,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":74128,"parent_record_offset":74068,"ancestor_record_offset":73928,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":74244,"parent_record_offset":74216,"ancestor_record_offset":73928,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":75076,"parent_record_offset":75052,"ancestor_record_offset":73928,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":74100,"parent_record_offset":74068,"ancestor_record_offset":73928,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":319,"record_offset":74152,"parent_record_offset":74128,"ancestor_record_offset":73928,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::menu_button_with_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":319,"record_offset":74184,"parent_record_offset":74068,"ancestor_record_offset":73928,"inlinee":17369,"source_line":72,"archive":"_pdb/A/modules/00319.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::menu_button_with_id::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43D9
// 首条位置: {"module":319,"inlinee":17369}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::image<ref$<str$> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":58,"record_offset":7192,"rva":521552,"va":5369230672}
// 编译实例 RVAs: 0x7f550, 0xa5b0e0, 0x1aa5b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f0d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e8ddfaffff","size":5,"soft_fail":false,"target_va":5369229520,"va":5369230830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t385300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e814e10500","size":5,"soft_fail":false,"target_va":5369616080,"va":5369230775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t385676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e88ce20500","size":5,"soft_fail":false,"target_va":5369616480,"va":5369230799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9950244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379567840,"raw_hex":"e8dc2b68ff","size":5,"soft_fail":false,"target_va":5369617648,"va":5379567887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t582817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e8a1e40800","size":5,"soft_fail":false,"target_va":5369813552,"va":5369230730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037b5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24291578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396650896,"raw_hex":"e806578dfe","size":5,"soft_fail":false,"target_va":5372360144,"va":5396651717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3518831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e86fb13500","size":5,"soft_fail":false,"target_va":5372749696,"va":5369230860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3518785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e841b13500","size":5,"soft_fail":false,"target_va":5372749712,"va":5369230922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23504509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396650896,"raw_hex":"e8835999fe","size":5,"soft_fail":false,"target_va":5373146800,"va":5396651304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3928080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e810f03b00","size":5,"soft_fail":false,"target_va":5373158800,"va":5369230715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14049bfb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23108780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396650896,"raw_hex":"e854639ffe","size":5,"soft_fail":false,"target_va":5373542320,"va":5396651095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ae20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379567840,"raw_hex":"e8a0fcffff","size":5,"soft_fail":false,"target_va":5379567136,"va":5379567995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc7190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3282295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396650896,"raw_hex":"e877153200","size":5,"soft_fail":false,"target_va":5399933328,"va":5396651028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc7300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3282684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396650896,"raw_hex":"e8fc163200","size":5,"soft_fail":false,"target_va":5399933696,"va":5396651007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34362456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e858540c02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369230899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33747154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e8d2f00202","size":5,"soft_fail":false,"target_va":5402978144,"va":5369230985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8296143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e8cf967e00","size":5,"soft_fail":false,"target_va":5377527120,"va":5369230972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6327304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396650896,"raw_hex":"e8088c6000","size":5,"soft_fail":false,"target_va":5402978912,"va":5396651603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t3930049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230672,"raw_hex":"e8c1f73b00","size":5,"soft_fail":false,"target_va":5373160864,"va":5369230810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396650896,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5396651405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":7328,"parent_record_offset":7192,"ancestor_record_offset":7192,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":109968,"parent_record_offset":109828,"ancestor_record_offset":109828,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":7192,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":7192,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":7192,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3457,"record_offset":4088,"parent_record_offset":4052,"ancestor_record_offset":2472,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":7808,"parent_record_offset":7772,"ancestor_record_offset":7192,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":8212,"parent_record_offset":8156,"ancestor_record_offset":7192,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3457,"record_offset":4052,"parent_record_offset":3932,"ancestor_record_offset":2472,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":8184,"parent_record_offset":8156,"ancestor_record_offset":7192,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":7772,"parent_record_offset":7744,"ancestor_record_offset":7192,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":7744,"parent_record_offset":7716,"ancestor_record_offset":7192,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":7928,"parent_record_offset":7744,"ancestor_record_offset":7192,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":8156,"parent_record_offset":8128,"ancestor_record_offset":7192,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7680,"parent_record_offset":7652,"ancestor_record_offset":7192,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7652,"parent_record_offset":7624,"ancestor_record_offset":7192,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7596,"parent_record_offset":7568,"ancestor_record_offset":7192,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7624,"parent_record_offset":7596,"ancestor_record_offset":7192,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7568,"parent_record_offset":7540,"ancestor_record_offset":7192,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":58,"record_offset":7540,"parent_record_offset":7512,"ancestor_record_offset":7192,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":58,"record_offset":7512,"parent_record_offset":7484,"ancestor_record_offset":7192,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3457,"record_offset":3964,"parent_record_offset":3932,"ancestor_record_offset":2472,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":58,"record_offset":7716,"parent_record_offset":7680,"ancestor_record_offset":7192,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":3457,"record_offset":4252,"parent_record_offset":2732,"ancestor_record_offset":2472,"inlinee":8015,"source_line":263,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":3457,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":2472,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3457,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":2472,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":58,"record_offset":7484,"parent_record_offset":7448,"ancestor_record_offset":7192,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3457,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":2472,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3457,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2472,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":58,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":7192,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3457,"record_offset":3236,"parent_record_offset":3212,"ancestor_record_offset":2472,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3457,"record_offset":4308,"parent_record_offset":2732,"ancestor_record_offset":2472,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3457,"record_offset":3900,"parent_record_offset":2732,"ancestor_record_offset":2472,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":58,"record_offset":7968,"parent_record_offset":7680,"ancestor_record_offset":7192,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":8056,"parent_record_offset":7448,"ancestor_record_offset":7192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3457,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":2472,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3457,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":2472,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":7884,"parent_record_offset":7856,"ancestor_record_offset":7192,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3457,"record_offset":3992,"parent_record_offset":3964,"ancestor_record_offset":2472,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3457,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":2472,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3457,"record_offset":3548,"parent_record_offset":3524,"ancestor_record_offset":2472,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3457,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":2472,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3457,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":2472,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3457,"record_offset":3524,"parent_record_offset":3304,"ancestor_record_offset":2472,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3457,"record_offset":2864,"parent_record_offset":2832,"ancestor_record_offset":2472,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":2472,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":3412,"parent_record_offset":3304,"ancestor_record_offset":2472,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":2472,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":3732,"parent_record_offset":2944,"ancestor_record_offset":2472,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":2472,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2472,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":2892,"parent_record_offset":2832,"ancestor_record_offset":2472,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":2944,"parent_record_offset":2916,"ancestor_record_offset":2472,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3457,"record_offset":3184,"parent_record_offset":3156,"ancestor_record_offset":2472,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3457,"record_offset":3212,"parent_record_offset":3184,"ancestor_record_offset":2472,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3457,"record_offset":3272,"parent_record_offset":3156,"ancestor_record_offset":2472,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3457,"record_offset":3156,"parent_record_offset":2944,"ancestor_record_offset":2472,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":2988,"parent_record_offset":2944,"ancestor_record_offset":2472,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3457,"record_offset":3304,"parent_record_offset":2944,"ancestor_record_offset":2472,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3457,"record_offset":3016,"parent_record_offset":2944,"ancestor_record_offset":2472,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3457,"record_offset":3100,"parent_record_offset":2944,"ancestor_record_offset":2472,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3457,"record_offset":3640,"parent_record_offset":2944,"ancestor_record_offset":2472,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3457,"record_offset":2784,"parent_record_offset":2732,"ancestor_record_offset":2472,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":7392,"parent_record_offset":7328,"ancestor_record_offset":7192,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::get_content_mut_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3457,"record_offset":2732,"parent_record_offset":2608,"ancestor_record_offset":2472,"inlinee":8013,"source_line":867,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$15::get_content_mut_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3457,"record_offset":3932,"parent_record_offset":3900,"ancestor_record_offset":2472,"inlinee":8017,"source_line":876,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3457,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":2472,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/03457.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":7364,"parent_record_offset":7328,"ancestor_record_offset":7192,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":7192,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::image::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":58,"record_offset":7448,"parent_record_offset":7328,"ancestor_record_offset":7192,"inlinee":8024,"source_line":80,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::image<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":58,"record_offset":8300,"rva":521872,"va":5369230992}
// 编译实例 RVAs: 0x7f690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f0d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e8a7f9ffff","size":5,"soft_fail":false,"target_va":5369229520,"va":5369231140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t384990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e8dedf0500","size":5,"soft_fail":false,"target_va":5369616080,"va":5369231085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t385366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e856e10500","size":5,"soft_fail":false,"target_va":5369616480,"va":5369231109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t582522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e87ae30800","size":5,"soft_fail":false,"target_va":5369813552,"va":5369231025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3518521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e839b03500","size":5,"soft_fail":false,"target_va":5372749696,"va":5369231170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3518475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e80bb03500","size":5,"soft_fail":false,"target_va":5372749712,"va":5369231232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34362146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e822530c02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369231209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33746844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e89cef0202","size":5,"soft_fail":false,"target_va":5402978144,"va":5369231295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8295833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e899957e00","size":5,"soft_fail":false,"target_va":5377527120,"va":5369231282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3929739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369230992,"raw_hex":"e88bf63b00","size":5,"soft_fail":false,"target_va":5373160864,"va":5369231120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":8428,"parent_record_offset":8300,"ancestor_record_offset":8300,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":8932,"parent_record_offset":8908,"ancestor_record_offset":8300,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":8300,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":8300,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":8908,"parent_record_offset":8872,"ancestor_record_offset":8300,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":9312,"parent_record_offset":9256,"ancestor_record_offset":8300,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":9284,"parent_record_offset":9256,"ancestor_record_offset":8300,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":8872,"parent_record_offset":8844,"ancestor_record_offset":8300,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":8844,"parent_record_offset":8816,"ancestor_record_offset":8300,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":9028,"parent_record_offset":8844,"ancestor_record_offset":8300,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":9256,"parent_record_offset":9228,"ancestor_record_offset":8300,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":8780,"parent_record_offset":8752,"ancestor_record_offset":8300,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":8300,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":8696,"parent_record_offset":8668,"ancestor_record_offset":8300,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":8724,"parent_record_offset":8696,"ancestor_record_offset":8300,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":8668,"parent_record_offset":8640,"ancestor_record_offset":8300,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":58,"record_offset":8640,"parent_record_offset":8612,"ancestor_record_offset":8300,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":58,"record_offset":8612,"parent_record_offset":8584,"ancestor_record_offset":8300,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":58,"record_offset":8816,"parent_record_offset":8780,"ancestor_record_offset":8300,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":58,"record_offset":8584,"parent_record_offset":8548,"ancestor_record_offset":8300,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":58,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":8300,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":58,"record_offset":9068,"parent_record_offset":8780,"ancestor_record_offset":8300,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":9156,"parent_record_offset":8548,"ancestor_record_offset":8300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":8984,"parent_record_offset":8956,"ancestor_record_offset":8300,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":8492,"parent_record_offset":8428,"ancestor_record_offset":8300,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":8464,"parent_record_offset":8428,"ancestor_record_offset":8300,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":8300,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::image::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":58,"record_offset":8548,"parent_record_offset":8428,"ancestor_record_offset":8300,"inlinee":8024,"source_line":80,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F58
// 首条位置: {"module":58,"inlinee":8024}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":58,"record_offset":7484,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":8056,"parent_record_offset":7448,"ancestor_record_offset":7448,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":7448,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":7856,"parent_record_offset":7832,"ancestor_record_offset":7448,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":7448,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":7808,"parent_record_offset":7772,"ancestor_record_offset":7448,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":8212,"parent_record_offset":8156,"ancestor_record_offset":7448,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":8184,"parent_record_offset":8156,"ancestor_record_offset":7448,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":7772,"parent_record_offset":7744,"ancestor_record_offset":7448,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":7744,"parent_record_offset":7716,"ancestor_record_offset":7448,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":7928,"parent_record_offset":7744,"ancestor_record_offset":7448,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":58,"record_offset":8156,"parent_record_offset":8128,"ancestor_record_offset":7448,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7680,"parent_record_offset":7652,"ancestor_record_offset":7448,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7652,"parent_record_offset":7624,"ancestor_record_offset":7448,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7596,"parent_record_offset":7568,"ancestor_record_offset":7448,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7624,"parent_record_offset":7596,"ancestor_record_offset":7448,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":58,"record_offset":7568,"parent_record_offset":7540,"ancestor_record_offset":7448,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":58,"record_offset":7540,"parent_record_offset":7512,"ancestor_record_offset":7448,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":58,"record_offset":7512,"parent_record_offset":7484,"ancestor_record_offset":7448,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":58,"record_offset":7716,"parent_record_offset":7680,"ancestor_record_offset":7448,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":58,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":7448,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":58,"record_offset":7968,"parent_record_offset":7680,"ancestor_record_offset":7448,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":7448,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":7884,"parent_record_offset":7856,"ancestor_record_offset":7448,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$0::frame
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6F1
// 首条位置: {"module":1025,"inlinee":50929}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1025,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1876,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/01025.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1025,"record_offset":1932,"parent_record_offset":1900,"ancestor_record_offset":1876,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/01025.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::core::NodeContext::sounds
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=22
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4328
// 首条位置: {"module":318,"inlinee":17192}
// 编译实例 RVAs: 0x443910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3566713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180176,"raw_hex":"e88793c9ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373180228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29798462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373180176,"raw_hex":"e83eb0c601","size":5,"soft_fail":false,"target_va":5402978880,"va":5373180413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":5584,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=16
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":4524,"parent_record_offset":4496,"ancestor_record_offset":4496,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":4496,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":4804,"parent_record_offset":4780,"ancestor_record_offset":4496,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":4496,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":4496,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4496,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":318,"record_offset":5552,"parent_record_offset":4524,"ancestor_record_offset":4496,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":4496,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4496,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":4496,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":4496,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":4496,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":5288,"parent_record_offset":5068,"ancestor_record_offset":4496,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":4632,"parent_record_offset":4600,"ancestor_record_offset":4496,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":4496,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5176,"parent_record_offset":5068,"ancestor_record_offset":4496,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":4496,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5496,"parent_record_offset":4708,"ancestor_record_offset":4496,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":4496,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":4496,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":4660,"parent_record_offset":4600,"ancestor_record_offset":4496,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":4496,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":4948,"parent_record_offset":4920,"ancestor_record_offset":4496,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":4976,"parent_record_offset":4948,"ancestor_record_offset":4496,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":5036,"parent_record_offset":4920,"ancestor_record_offset":4496,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":4920,"parent_record_offset":4708,"ancestor_record_offset":4496,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":4752,"parent_record_offset":4708,"ancestor_record_offset":4496,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5068,"parent_record_offset":4708,"ancestor_record_offset":4496,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":4780,"parent_record_offset":4708,"ancestor_record_offset":4496,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=28
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":4864,"parent_record_offset":4708,"ancestor_record_offset":4496,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=14
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":5404,"parent_record_offset":4708,"ancestor_record_offset":4496,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":4552,"parent_record_offset":4524,"ancestor_record_offset":4496,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":4496,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$23::deref
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6DA
// 首条位置: {"module":1018,"inlinee":50906}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":80688,"parent_record_offset":80656,"ancestor_record_offset":80656,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":81544,"parent_record_offset":81520,"ancestor_record_offset":80656,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":81012,"parent_record_offset":80988,"ancestor_record_offset":80656,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":81096,"parent_record_offset":81072,"ancestor_record_offset":80656,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":81036,"parent_record_offset":81012,"ancestor_record_offset":80656,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":81208,"parent_record_offset":81184,"ancestor_record_offset":80656,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":81456,"parent_record_offset":81432,"ancestor_record_offset":80656,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":81348,"parent_record_offset":81324,"ancestor_record_offset":80656,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":81520,"parent_record_offset":81496,"ancestor_record_offset":80656,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":80788,"parent_record_offset":80752,"ancestor_record_offset":80656,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":80812,"parent_record_offset":80788,"ancestor_record_offset":80656,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":81496,"parent_record_offset":81276,"ancestor_record_offset":80656,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":80840,"parent_record_offset":80812,"ancestor_record_offset":80656,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":81408,"parent_record_offset":81384,"ancestor_record_offset":80656,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":81384,"parent_record_offset":81276,"ancestor_record_offset":80656,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":81324,"parent_record_offset":81300,"ancestor_record_offset":80656,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":81704,"parent_record_offset":80916,"ancestor_record_offset":80656,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":81300,"parent_record_offset":81276,"ancestor_record_offset":80656,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":80892,"parent_record_offset":80868,"ancestor_record_offset":80656,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":80868,"parent_record_offset":80812,"ancestor_record_offset":80656,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":80916,"parent_record_offset":80892,"ancestor_record_offset":80656,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":81156,"parent_record_offset":81128,"ancestor_record_offset":80656,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":81184,"parent_record_offset":81156,"ancestor_record_offset":80656,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":81244,"parent_record_offset":81128,"ancestor_record_offset":80656,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":81128,"parent_record_offset":80916,"ancestor_record_offset":80656,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":80960,"parent_record_offset":80916,"ancestor_record_offset":80656,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":81276,"parent_record_offset":80916,"ancestor_record_offset":80656,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":80988,"parent_record_offset":80916,"ancestor_record_offset":80656,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":81072,"parent_record_offset":80916,"ancestor_record_offset":80656,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":81612,"parent_record_offset":80916,"ancestor_record_offset":80656,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":80752,"parent_record_offset":80720,"ancestor_record_offset":80656,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":80720,"parent_record_offset":80688,"ancestor_record_offset":80656,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":81568,"parent_record_offset":81544,"ancestor_record_offset":80656,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$26::deref
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4320
// 首条位置: {"module":318,"inlinee":17184}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=28
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":548,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":548,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":548,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":548,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":548,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":57752,"parent_record_offset":57728,"ancestor_record_offset":57680,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":548,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":548,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2499,"record_offset":13732,"parent_record_offset":13700,"ancestor_record_offset":13668,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/02499.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":548,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":668,"parent_record_offset":632,"ancestor_record_offset":548,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":548,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":1380,"parent_record_offset":1160,"ancestor_record_offset":548,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":724,"parent_record_offset":692,"ancestor_record_offset":548,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":548,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1268,"parent_record_offset":1160,"ancestor_record_offset":548,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":548,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1588,"parent_record_offset":800,"ancestor_record_offset":548,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":548,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":548,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":752,"parent_record_offset":692,"ancestor_record_offset":548,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":548,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":1040,"parent_record_offset":1012,"ancestor_record_offset":548,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":1068,"parent_record_offset":1040,"ancestor_record_offset":548,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":1128,"parent_record_offset":1012,"ancestor_record_offset":548,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":1012,"parent_record_offset":800,"ancestor_record_offset":548,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":844,"parent_record_offset":800,"ancestor_record_offset":548,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":1160,"parent_record_offset":800,"ancestor_record_offset":548,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":872,"parent_record_offset":800,"ancestor_record_offset":548,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=46
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":956,"parent_record_offset":800,"ancestor_record_offset":548,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=23
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":1496,"parent_record_offset":800,"ancestor_record_offset":548,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":632,"parent_record_offset":604,"ancestor_record_offset":548,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":548,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=23
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":548,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$32::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":16416,"rva":4440736,"va":5373149856}
// 编译实例 RVAs: 0x43c2a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3536372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373149856,"raw_hex":"e80c0acaff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373149887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29828828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373149856,"raw_hex":"e8dc26c701","size":5,"soft_fail":false,"target_va":5402978912,"va":5373150079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":16540,"parent_record_offset":16416,"ancestor_record_offset":16416,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":16416,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":16896,"parent_record_offset":16872,"ancestor_record_offset":16416,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":16980,"parent_record_offset":16956,"ancestor_record_offset":16416,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":16920,"parent_record_offset":16896,"ancestor_record_offset":16416,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":17084,"parent_record_offset":17060,"ancestor_record_offset":16416,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":17332,"parent_record_offset":17308,"ancestor_record_offset":16416,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":16416,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":318,"record_offset":16568,"parent_record_offset":16540,"ancestor_record_offset":16416,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":17396,"parent_record_offset":17372,"ancestor_record_offset":16416,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":16664,"parent_record_offset":16628,"ancestor_record_offset":16416,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":16692,"parent_record_offset":16664,"ancestor_record_offset":16416,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":17372,"parent_record_offset":17152,"ancestor_record_offset":16416,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":16724,"parent_record_offset":16692,"ancestor_record_offset":16416,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17284,"parent_record_offset":17260,"ancestor_record_offset":16416,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17260,"parent_record_offset":17152,"ancestor_record_offset":16416,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":16416,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17580,"parent_record_offset":16800,"ancestor_record_offset":16416,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":16416,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":16776,"parent_record_offset":16752,"ancestor_record_offset":16416,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":16752,"parent_record_offset":16692,"ancestor_record_offset":16416,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":16800,"parent_record_offset":16776,"ancestor_record_offset":16416,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":17036,"parent_record_offset":17012,"ancestor_record_offset":16416,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":16416,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":17120,"parent_record_offset":17012,"ancestor_record_offset":16416,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":17012,"parent_record_offset":16800,"ancestor_record_offset":16416,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":16844,"parent_record_offset":16800,"ancestor_record_offset":16416,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17152,"parent_record_offset":16800,"ancestor_record_offset":16416,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":16872,"parent_record_offset":16800,"ancestor_record_offset":16416,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":16956,"parent_record_offset":16800,"ancestor_record_offset":16416,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":17488,"parent_record_offset":16800,"ancestor_record_offset":16416,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":16628,"parent_record_offset":16596,"ancestor_record_offset":16416,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":16596,"parent_record_offset":16540,"ancestor_record_offset":16416,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":17444,"parent_record_offset":17420,"ancestor_record_offset":16416,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$40::deref
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14689
// 首条位置: {"module":2468,"inlinee":83593}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":28384,"parent_record_offset":28356,"ancestor_record_offset":28356,"inlinee":17185,"source_line":934,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":29232,"parent_record_offset":29208,"ancestor_record_offset":28356,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":28700,"parent_record_offset":28676,"ancestor_record_offset":28356,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":28784,"parent_record_offset":28760,"ancestor_record_offset":28356,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":28724,"parent_record_offset":28700,"ancestor_record_offset":28356,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":28896,"parent_record_offset":28872,"ancestor_record_offset":28356,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2468,"record_offset":115776,"parent_record_offset":115752,"ancestor_record_offset":115704,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":29144,"parent_record_offset":29120,"ancestor_record_offset":28356,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":29036,"parent_record_offset":29012,"ancestor_record_offset":28356,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":29208,"parent_record_offset":29184,"ancestor_record_offset":28356,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":28468,"parent_record_offset":28440,"ancestor_record_offset":28356,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":28496,"parent_record_offset":28468,"ancestor_record_offset":28356,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":29184,"parent_record_offset":28964,"ancestor_record_offset":28356,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":28528,"parent_record_offset":28496,"ancestor_record_offset":28356,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":29096,"parent_record_offset":29072,"ancestor_record_offset":28356,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":29072,"parent_record_offset":28964,"ancestor_record_offset":28356,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":29012,"parent_record_offset":28988,"ancestor_record_offset":28356,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":29392,"parent_record_offset":28604,"ancestor_record_offset":28356,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":28988,"parent_record_offset":28964,"ancestor_record_offset":28356,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":28356,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":28556,"parent_record_offset":28496,"ancestor_record_offset":28356,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":28604,"parent_record_offset":28580,"ancestor_record_offset":28356,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":28844,"parent_record_offset":28816,"ancestor_record_offset":28356,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":28872,"parent_record_offset":28844,"ancestor_record_offset":28356,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":28932,"parent_record_offset":28816,"ancestor_record_offset":28356,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":28816,"parent_record_offset":28604,"ancestor_record_offset":28356,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":28648,"parent_record_offset":28604,"ancestor_record_offset":28356,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":28964,"parent_record_offset":28604,"ancestor_record_offset":28356,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":28676,"parent_record_offset":28604,"ancestor_record_offset":28356,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=16
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":28760,"parent_record_offset":28604,"ancestor_record_offset":28356,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":29300,"parent_record_offset":28604,"ancestor_record_offset":28356,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":28440,"parent_record_offset":28412,"ancestor_record_offset":28356,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":28412,"parent_record_offset":28384,"ancestor_record_offset":28356,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":29256,"parent_record_offset":29232,"ancestor_record_offset":28356,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$24::deref_mut
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6D8
// 首条位置: {"module":1018,"inlinee":50904}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77668,"parent_record_offset":77628,"ancestor_record_offset":77628,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":78524,"parent_record_offset":78500,"ancestor_record_offset":77628,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":77992,"parent_record_offset":77968,"ancestor_record_offset":77628,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":78076,"parent_record_offset":78052,"ancestor_record_offset":77628,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":78016,"parent_record_offset":77992,"ancestor_record_offset":77628,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":78188,"parent_record_offset":78164,"ancestor_record_offset":77628,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":78740,"parent_record_offset":77668,"ancestor_record_offset":77628,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":78436,"parent_record_offset":78412,"ancestor_record_offset":77628,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":78328,"parent_record_offset":78304,"ancestor_record_offset":77628,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":78500,"parent_record_offset":78476,"ancestor_record_offset":77628,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":77780,"parent_record_offset":77744,"ancestor_record_offset":77628,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":77744,"parent_record_offset":77708,"ancestor_record_offset":77628,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":78476,"parent_record_offset":78256,"ancestor_record_offset":77628,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":77820,"parent_record_offset":77780,"ancestor_record_offset":77628,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":78388,"parent_record_offset":78364,"ancestor_record_offset":77628,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":78364,"parent_record_offset":78256,"ancestor_record_offset":77628,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":78304,"parent_record_offset":78280,"ancestor_record_offset":77628,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":78684,"parent_record_offset":77896,"ancestor_record_offset":77628,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":78280,"parent_record_offset":78256,"ancestor_record_offset":77628,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":77872,"parent_record_offset":77848,"ancestor_record_offset":77628,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":77848,"parent_record_offset":77780,"ancestor_record_offset":77628,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":77896,"parent_record_offset":77872,"ancestor_record_offset":77628,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":78136,"parent_record_offset":78108,"ancestor_record_offset":77628,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":78164,"parent_record_offset":78136,"ancestor_record_offset":77628,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":78224,"parent_record_offset":78108,"ancestor_record_offset":77628,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":78108,"parent_record_offset":77896,"ancestor_record_offset":77628,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":77940,"parent_record_offset":77896,"ancestor_record_offset":77628,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":78256,"parent_record_offset":77896,"ancestor_record_offset":77628,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":77968,"parent_record_offset":77896,"ancestor_record_offset":77628,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":78052,"parent_record_offset":77896,"ancestor_record_offset":77628,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":78592,"parent_record_offset":77896,"ancestor_record_offset":77628,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":77708,"parent_record_offset":77668,"ancestor_record_offset":77628,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":78548,"parent_record_offset":78524,"ancestor_record_offset":77628,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$30::deref_mut
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xC6CE
// 首条位置: {"module":1018,"inlinee":50894}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":60316,"parent_record_offset":60288,"ancestor_record_offset":60288,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1018,"record_offset":61136,"parent_record_offset":61112,"ancestor_record_offset":60288,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":60604,"parent_record_offset":60580,"ancestor_record_offset":60288,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1018,"record_offset":60688,"parent_record_offset":60664,"ancestor_record_offset":60288,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1018,"record_offset":60628,"parent_record_offset":60604,"ancestor_record_offset":60288,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1018,"record_offset":60800,"parent_record_offset":60776,"ancestor_record_offset":60288,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":61048,"parent_record_offset":61024,"ancestor_record_offset":60288,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1018,"record_offset":60940,"parent_record_offset":60916,"ancestor_record_offset":60288,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1018,"record_offset":61112,"parent_record_offset":61088,"ancestor_record_offset":60288,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":60400,"parent_record_offset":60372,"ancestor_record_offset":60288,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":60372,"parent_record_offset":60344,"ancestor_record_offset":60288,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":61088,"parent_record_offset":60868,"ancestor_record_offset":60288,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1018,"record_offset":60432,"parent_record_offset":60400,"ancestor_record_offset":60288,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":61000,"parent_record_offset":60976,"ancestor_record_offset":60288,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60976,"parent_record_offset":60868,"ancestor_record_offset":60288,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60916,"parent_record_offset":60892,"ancestor_record_offset":60288,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":61296,"parent_record_offset":60508,"ancestor_record_offset":60288,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60892,"parent_record_offset":60868,"ancestor_record_offset":60288,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60484,"parent_record_offset":60460,"ancestor_record_offset":60288,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60460,"parent_record_offset":60400,"ancestor_record_offset":60288,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60508,"parent_record_offset":60484,"ancestor_record_offset":60288,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":60748,"parent_record_offset":60720,"ancestor_record_offset":60288,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":60776,"parent_record_offset":60748,"ancestor_record_offset":60288,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":60836,"parent_record_offset":60720,"ancestor_record_offset":60288,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1018,"record_offset":60720,"parent_record_offset":60508,"ancestor_record_offset":60288,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60552,"parent_record_offset":60508,"ancestor_record_offset":60288,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1018,"record_offset":60868,"parent_record_offset":60508,"ancestor_record_offset":60288,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":60580,"parent_record_offset":60508,"ancestor_record_offset":60288,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":60664,"parent_record_offset":60508,"ancestor_record_offset":60288,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1018,"record_offset":61204,"parent_record_offset":60508,"ancestor_record_offset":60288,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1018,"record_offset":60344,"parent_record_offset":60316,"ancestor_record_offset":60288,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":61160,"parent_record_offset":61136,"ancestor_record_offset":60288,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$33::deref_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":17648,"rva":4440976,"va":5373150096}
// 编译实例 RVAs: 0x43c390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3536612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373150096,"raw_hex":"e81c09caff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373150127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29828568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373150096,"raw_hex":"e8d825c701","size":5,"soft_fail":false,"target_va":5402978880,"va":5373150307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":17776,"parent_record_offset":17648,"ancestor_record_offset":17648,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":18616,"parent_record_offset":18592,"ancestor_record_offset":17648,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":18092,"parent_record_offset":18068,"ancestor_record_offset":17648,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":17648,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":17648,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":18280,"parent_record_offset":18256,"ancestor_record_offset":17648,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":318,"record_offset":18832,"parent_record_offset":17776,"ancestor_record_offset":17648,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":17648,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":18420,"parent_record_offset":18396,"ancestor_record_offset":17648,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":318,"record_offset":17808,"parent_record_offset":17776,"ancestor_record_offset":17648,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":18592,"parent_record_offset":18568,"ancestor_record_offset":17648,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":17888,"parent_record_offset":17860,"ancestor_record_offset":17648,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":17860,"parent_record_offset":17832,"ancestor_record_offset":17648,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":18568,"parent_record_offset":18348,"ancestor_record_offset":17648,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":17920,"parent_record_offset":17888,"ancestor_record_offset":17648,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":18480,"parent_record_offset":18456,"ancestor_record_offset":17648,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":18456,"parent_record_offset":18348,"ancestor_record_offset":17648,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":18396,"parent_record_offset":18372,"ancestor_record_offset":17648,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":18776,"parent_record_offset":17996,"ancestor_record_offset":17648,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":18372,"parent_record_offset":18348,"ancestor_record_offset":17648,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17972,"parent_record_offset":17948,"ancestor_record_offset":17648,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17948,"parent_record_offset":17888,"ancestor_record_offset":17648,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":17996,"parent_record_offset":17972,"ancestor_record_offset":17648,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":18232,"parent_record_offset":18208,"ancestor_record_offset":17648,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":18256,"parent_record_offset":18232,"ancestor_record_offset":17648,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":18316,"parent_record_offset":18208,"ancestor_record_offset":17648,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":18208,"parent_record_offset":17996,"ancestor_record_offset":17648,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":18040,"parent_record_offset":17996,"ancestor_record_offset":17648,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":18348,"parent_record_offset":17996,"ancestor_record_offset":17648,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":18068,"parent_record_offset":17996,"ancestor_record_offset":17648,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":18152,"parent_record_offset":17996,"ancestor_record_offset":17648,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":18684,"parent_record_offset":17996,"ancestor_record_offset":17648,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":17832,"parent_record_offset":17776,"ancestor_record_offset":17648,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":18640,"parent_record_offset":18616,"ancestor_record_offset":17648,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::MenuButtonView::into_inner
// 记录形式: compiled_procedure; instances=1
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":16288,"rva":4440720,"va":5373149840}
// 编译实例 RVAs: 0x43c290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::MenuButtonView::on_click
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4342
// 首条位置: {"module":318,"record_offset":4364,"rva":4436384,"va":5373145504}
// 编译实例 RVAs: 0x43b1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3532036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145504,"raw_hex":"e8fc1acaff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373145551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29833135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145504,"raw_hex":"e8af37c701","size":5,"soft_fail":false,"target_va":5402978880,"va":5373145740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":318,"record_offset":4496,"parent_record_offset":4364,"ancestor_record_offset":4364,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":4364,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":4804,"parent_record_offset":4780,"ancestor_record_offset":4364,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":4364,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":4364,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":4364,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":318,"record_offset":5552,"parent_record_offset":4524,"ancestor_record_offset":4364,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":4364,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4364,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":4364,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":4364,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":4364,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":5288,"parent_record_offset":5068,"ancestor_record_offset":4364,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":4632,"parent_record_offset":4600,"ancestor_record_offset":4364,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":4364,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5176,"parent_record_offset":5068,"ancestor_record_offset":4364,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":4364,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5496,"parent_record_offset":4708,"ancestor_record_offset":4364,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":4364,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":4364,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":4660,"parent_record_offset":4600,"ancestor_record_offset":4364,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":4364,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":4948,"parent_record_offset":4920,"ancestor_record_offset":4364,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":4976,"parent_record_offset":4948,"ancestor_record_offset":4364,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":5036,"parent_record_offset":4920,"ancestor_record_offset":4364,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":4920,"parent_record_offset":4708,"ancestor_record_offset":4364,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":4752,"parent_record_offset":4708,"ancestor_record_offset":4364,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":5068,"parent_record_offset":4708,"ancestor_record_offset":4364,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":4780,"parent_record_offset":4708,"ancestor_record_offset":4364,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":4864,"parent_record_offset":4708,"ancestor_record_offset":4364,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":5404,"parent_record_offset":4708,"ancestor_record_offset":4364,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":4552,"parent_record_offset":4524,"ancestor_record_offset":4364,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":5584,"parent_record_offset":4496,"ancestor_record_offset":4364,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":4524,"parent_record_offset":4496,"ancestor_record_offset":4364,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":4364,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::MenuButtonView::weight
// 记录形式: compiled_procedure; instances=1
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":5620,"rva":4436640,"va":5373145760}
// 编译实例 RVAs: 0x43b2a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6420626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145760,"raw_hex":"e892f86100","size":5,"soft_fail":false,"target_va":5379566416,"va":5373145785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145760,"raw_hex":"e8183b2c00","size":5,"soft_fail":false,"target_va":5376044512,"va":5373145795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":5752,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::MenuButtonView::max_width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4344
// 首条位置: {"module":318,"record_offset":5784,"rva":4436704,"va":5373145824}
// 编译实例 RVAs: 0x43b2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6420558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145824,"raw_hex":"e84ef86100","size":5,"soft_fail":false,"target_va":5379566416,"va":5373145853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145824,"raw_hex":"e8ec3a2c00","size":5,"soft_fail":false,"target_va":5376044544,"va":5373145871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":318,"record_offset":5948,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":5920,"parent_record_offset":5784,"ancestor_record_offset":5784,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::MenuButtonView::align
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4345
// 首条位置: {"module":318,"record_offset":5980,"rva":4436784,"va":5373145904}
// 编译实例 RVAs: 0x43b330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6420482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145904,"raw_hex":"e802f86100","size":5,"soft_fail":false,"target_va":5379566416,"va":5373145929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145904,"raw_hex":"e8283b2c00","size":5,"soft_fail":false,"target_va":5376044672,"va":5373145939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":6108,"parent_record_offset":5980,"ancestor_record_offset":5980,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::LabelView::metrics
// 记录形式: inline_instance, inlinee_declaration; instances=76
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E0
// 首条位置: {"module":15,"inlinee":5600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=23
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":77476,"parent_record_offset":77448,"ancestor_record_offset":77448,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::LabelView::metrics<f32>
// 记录形式: compiled_procedure; instances=2
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":31,"record_offset":28560,"rva":333296,"va":5369042416}
// 编译实例 RVAs: 0x515f0, 0x15ed080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t186304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369042416,"raw_hex":"e8c0d70200","size":5,"soft_fail":false,"target_va":5369228752,"va":5369042443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2256389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369042416,"raw_hex":"e8056e2200","size":5,"soft_fail":false,"target_va":5371298848,"va":5369042454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12136882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391700096,"raw_hex":"e84ece46ff","size":5,"soft_fail":false,"target_va":5379563248,"va":5391700125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5c1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12127965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391700096,"raw_hex":"e823f146ff","size":5,"soft_fail":false,"target_va":5379572176,"va":5391700136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2427,"record_offset":4184,"parent_record_offset":4052,"ancestor_record_offset":4052,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02427.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::LabelView::max_width
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4346
// 首条位置: {"module":318,"record_offset":6140,"rva":4436848,"va":5373145968}
// 编译实例 RVAs: 0x43b370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6417246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145968,"raw_hex":"e85eeb6100","size":5,"soft_fail":false,"target_va":5379563248,"va":5373145997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373145968,"raw_hex":"e85c3a2c00","size":5,"soft_fail":false,"target_va":5376044544,"va":5373146015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":318,"record_offset":6300,"parent_record_offset":6140,"ancestor_record_offset":6140,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":6272,"parent_record_offset":6140,"ancestor_record_offset":6140,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::LabelView::align
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=12
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4347
// 首条位置: {"module":318,"record_offset":6332,"rva":4436928,"va":5373146048}
// 编译实例 RVAs: 0x43b3c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6417170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146048,"raw_hex":"e812eb6100","size":5,"soft_fail":false,"target_va":5379563248,"va":5373146073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146048,"raw_hex":"e8983a2c00","size":5,"soft_fail":false,"target_va":5376044672,"va":5373146083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":6456,"parent_record_offset":6332,"ancestor_record_offset":6332,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::LabelView::family
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=17
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4348
// 首条位置: {"module":318,"record_offset":6488,"rva":4436992,"va":5373146112}
// 编译实例 RVAs: 0x43b400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6417106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146112,"raw_hex":"e8d2ea6100","size":5,"soft_fail":false,"target_va":5379563248,"va":5373146137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2897816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146112,"raw_hex":"e898372c00","size":5,"soft_fail":false,"target_va":5376043968,"va":5373146147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":6612,"parent_record_offset":6488,"ancestor_record_offset":6488,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::LabelView::weight
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=20
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4349
// 首条位置: {"module":318,"record_offset":6644,"rva":4437056,"va":5373146176}
// 编译实例 RVAs: 0x43b440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6417042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146176,"raw_hex":"e892ea6100","size":5,"soft_fail":false,"target_va":5379563248,"va":5373146201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_weight
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146176,"raw_hex":"e878392c00","size":5,"soft_fail":false,"target_va":5376044512,"va":5373146211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=16
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":6768,"parent_record_offset":6644,"ancestor_record_offset":6644,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::LabelView::color
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=18
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x434A
// 首条位置: {"module":318,"record_offset":6800,"rva":4437120,"va":5373146240}
// 编译实例 RVAs: 0x43b480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6416978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146240,"raw_hex":"e852ea6100","size":5,"soft_fail":false,"target_va":5379563248,"va":5373146265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":6924,"parent_record_offset":6800,"ancestor_record_offset":6800,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$4::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x434C
// 首条位置: {"module":1018,"inlinee":17228}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$5::prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":6956,"rva":4437184,"va":5373146304}
// 编译实例 RVAs: 0x43b4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::bounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146304,"raw_hex":"e8b1392c00","size":5,"soft_fail":false,"target_va":5376044688,"va":5373146330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::prepare
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146304,"raw_hex":"e8c9392c00","size":5,"soft_fail":false,"target_va":5376044704,"va":5373146322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$5::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":7072,"rva":4437264,"va":5373146384}
// 编译实例 RVAs: 0x43b510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146384,"raw_hex":"e8103b2c00","size":5,"soft_fail":false,"target_va":5376045152,"va":5373146443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::scale_u32_packed_srgba_color
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3797449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146384,"raw_hex":"e8c9f13900","size":5,"soft_fail":false,"target_va":5376943872,"va":5373146418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::MenuButtonView::metrics
// 记录形式: inline_instance, inlinee_declaration; instances=41
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E2
// 首条位置: {"module":15,"inlinee":5602}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":88752,"parent_record_offset":88724,"ancestor_record_offset":88724,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::MenuButtonView::color
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 295
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4352
// 首条位置: {"module":318,"record_offset":7188,"rva":4437344,"va":5373146464}
// 编译实例 RVAs: 0x43b560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6419922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146464,"raw_hex":"e8d2f56100","size":5,"soft_fail":false,"target_va":5379566416,"va":5373146489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":7316,"parent_record_offset":7188,"ancestor_record_offset":7188,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::MenuButtonView::force_highlight
// 记录形式: compiled_procedure; instances=1
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":7348,"rva":4437408,"va":5373146528}
// 编译实例 RVAs: 0x43b5a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ab50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6419858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146528,"raw_hex":"e892f56100","size":5,"soft_fail":false,"target_va":5379566416,"va":5373146553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":7488,"parent_record_offset":7348,"ancestor_record_offset":7348,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$7::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4355
// 首条位置: {"module":1018,"inlinee":17237}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$8::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 367
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":7520,"rva":4437472,"va":5373146592}
// 编译实例 RVAs: 0x43b5e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2898477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146592,"raw_hex":"e82d3a2c00","size":5,"soft_fail":false,"target_va":5376045152,"va":5373146670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::scale_u32_packed_srgba_color
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3797222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146592,"raw_hex":"e8e6f03900","size":5,"soft_fail":false,"target_va":5376943872,"va":5373146645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":318,"record_offset":7644,"parent_record_offset":7520,"ancestor_record_offset":7520,"inlinee":13076,"source_line":603,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::ImageView::tint
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4359
// 首条位置: {"module":318,"record_offset":7676,"rva":4437568,"va":5373146688}
// 编译实例 RVAs: 0x43b640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ae20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6420418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146688,"raw_hex":"e8c2f76100","size":5,"soft_fail":false,"target_va":5379567136,"va":5373146713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":7800,"parent_record_offset":7676,"ancestor_record_offset":7676,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::ImageView::blur_extend
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x435A
// 首条位置: {"module":318,"record_offset":7832,"rva":4437632,"va":5373146752}
// 编译实例 RVAs: 0x43b680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ae20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6420358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146752,"raw_hex":"e886f76100","size":5,"soft_fail":false,"target_va":5379567136,"va":5373146773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":7964,"parent_record_offset":7832,"ancestor_record_offset":7832,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$10::create_node_content
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x435C
// 首条位置: {"module":318,"record_offset":7996,"rva":4437680,"va":5373146800}
// 编译实例 RVAs: 0x43b6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1490001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146800,"raw_hex":"e8af43e9ff","size":5,"soft_fail":false,"target_va":5371656896,"va":5373146892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":8132,"parent_record_offset":7996,"ancestor_record_offset":7996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":318,"record_offset":8172,"parent_record_offset":8152,"ancestor_record_offset":7996,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":8152,"parent_record_offset":8132,"ancestor_record_offset":7996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":318,"record_offset":8204,"parent_record_offset":8172,"ancestor_record_offset":7996,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":318,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":7996,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":318,"record_offset":8260,"parent_record_offset":8172,"ancestor_record_offset":7996,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$11::prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 434
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":8304,"rva":4437792,"va":5373146912}
// 编译实例 RVAs: 0x43b720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-397557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e80beff9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373147264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056c370
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1248146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e8920b1300","size":5,"soft_fail":false,"target_va":5374395248,"va":5373147097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407bf410
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3685438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e83e3c3800","size":5,"soft_fail":false,"target_va":5376832528,"va":5373147085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6450049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e8816b6200","size":5,"soft_fail":false,"target_va":5379597488,"va":5373147434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a681b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6474168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e8b8c96200","size":5,"soft_fail":false,"target_va":5379621296,"va":5373147123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29831530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e86a31c701","size":5,"soft_fail":false,"target_va":5402978912,"va":5373147377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_handle_and_texture_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1207421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e87d6c1200","size":5,"soft_fail":false,"target_va":5374354416,"va":5373146990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1209878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e816761200","size":5,"soft_fail":false,"target_va":5374356896,"va":5373147013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3194728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e868bf3000","size":5,"soft_fail":false,"target_va":5376341760,"va":5373147027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6472620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e8acc36200","size":5,"soft_fail":false,"target_va":5379619792,"va":5373147167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::fill_color
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6472117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e8b5c16200","size":5,"soft_fail":false,"target_va":5379619264,"va":5373147142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3799432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373146912,"raw_hex":"e888f93900","size":5,"soft_fail":false,"target_va":5376946416,"va":5373146979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":318,"record_offset":8440,"parent_record_offset":8304,"ancestor_record_offset":8304,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":8592,"parent_record_offset":8304,"ancestor_record_offset":8304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$10::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":8504,"parent_record_offset":8304,"ancestor_record_offset":8304,"inlinee":14658,"source_line":982,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":318,"record_offset":8476,"parent_record_offset":8304,"ancestor_record_offset":8304,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":8304,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":8304,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":318,"record_offset":9004,"parent_record_offset":8976,"ancestor_record_offset":8304,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":318,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":8304,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":8304,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":318,"record_offset":9032,"parent_record_offset":9004,"ancestor_record_offset":8304,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":8532,"parent_record_offset":8504,"ancestor_record_offset":8304,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":8304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$11::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 478
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":9144,"rva":4438352,"va":5373147472}
// 编译实例 RVAs: 0x43b950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::rect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2889855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373147472,"raw_hex":"e87f182c00","size":5,"soft_fail":false,"target_va":5376037504,"va":5373147644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":318,"record_offset":9284,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":9564,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":9480,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":9312,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::RenderContext::draw_rect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":9648,"parent_record_offset":9144,"ancestor_record_offset":9144,"inlinee":17200,"source_line":1035,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":318,"record_offset":9676,"parent_record_offset":9648,"ancestor_record_offset":9144,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":9144,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":9144,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":9144,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":9732,"parent_record_offset":9648,"ancestor_record_offset":9144,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":9144,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::FrameView::draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 499
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":9884,"rva":4438544,"va":5373147664}
// 编译实例 RVAs: 0x43ba10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-398101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373147664,"raw_hex":"e8ebecf9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373147808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5a3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6416782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373147664,"raw_hex":"e88ee96100","size":5,"soft_fail":false,"target_va":5379564496,"va":5373147709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6449568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373147664,"raw_hex":"e8a0696200","size":5,"soft_fail":false,"target_va":5379597488,"va":5373147915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":10040,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":10012,"parent_record_offset":9884,"ancestor_record_offset":9884,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":9884,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":10292,"parent_record_offset":10268,"ancestor_record_offset":9884,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":318,"record_offset":10436,"parent_record_offset":10408,"ancestor_record_offset":9884,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":318,"record_offset":10268,"parent_record_offset":10244,"ancestor_record_offset":9884,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":10080,"parent_record_offset":10060,"ancestor_record_offset":9884,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":318,"record_offset":10464,"parent_record_offset":10436,"ancestor_record_offset":9884,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":10060,"parent_record_offset":10040,"ancestor_record_offset":9884,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$38::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 506
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4334
// 首条位置: {"module":318,"inlinee":17204}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$13::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 513
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4363
// 首条位置: {"module":1018,"inlinee":17251}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":101096,"parent_record_offset":101040,"ancestor_record_offset":101040,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$38::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":101068,"parent_record_offset":101040,"ancestor_record_offset":101040,"inlinee":17204,"source_line":506,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1018,"record_offset":101144,"parent_record_offset":101120,"ancestor_record_offset":101040,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1018,"record_offset":101120,"parent_record_offset":101096,"ancestor_record_offset":101040,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":101180,"parent_record_offset":101144,"ancestor_record_offset":101040,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":101204,"parent_record_offset":101180,"ancestor_record_offset":101040,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1018,"record_offset":101236,"parent_record_offset":101144,"ancestor_record_offset":101040,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$14::interactive
// 记录形式: compiled_procedure; instances=1
// 源码行: 522
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":10576,"rva":13200,"va":5368722320}
// 编译实例 RVAs: 0x3390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$14::prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 524
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":10696,"rva":4438816,"va":5373147936}
// 编译实例 RVAs: 0x43bb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407bf410
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3684562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373147936,"raw_hex":"e8d2383800","size":5,"soft_fail":false,"target_va":5376832528,"va":5373147961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6471804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373147936,"raw_hex":"e87cc06200","size":5,"soft_fail":false,"target_va":5379619792,"va":5373147983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":10820,"parent_record_offset":10696,"ancestor_record_offset":10696,"inlinee":17206,"source_line":977,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$14::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 532
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":10852,"rva":4438896,"va":5373148016}
// 编译实例 RVAs: 0x43bb70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a67b20
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6471583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148016,"raw_hex":"e89fbf6200","size":5,"soft_fail":false,"target_va":5379619616,"va":5373148028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::rect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2889352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148016,"raw_hex":"e888162c00","size":5,"soft_fail":false,"target_va":5376037504,"va":5373148147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":11156,"parent_record_offset":10852,"ancestor_record_offset":10852,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":11072,"parent_record_offset":10852,"ancestor_record_offset":10852,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":10988,"parent_record_offset":10852,"ancestor_record_offset":10852,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::RenderContext::draw_rect
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":11240,"parent_record_offset":10852,"ancestor_record_offset":10852,"inlinee":17200,"source_line":1035,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":318,"record_offset":11268,"parent_record_offset":11240,"ancestor_record_offset":10852,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":11012,"parent_record_offset":10988,"ancestor_record_offset":10852,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":11180,"parent_record_offset":11156,"ancestor_record_offset":10852,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":10852,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":11324,"parent_record_offset":11240,"ancestor_record_offset":10852,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":11292,"parent_record_offset":11268,"ancestor_record_offset":10852,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$39::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 571
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4338
// 首条位置: {"module":318,"inlinee":17208}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$18::pill_button<ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 617
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":58,"record_offset":9400,"rva":522192,"va":5369231312}
// 编译实例 RVAs: 0x7f7d0, 0xa5b230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t382029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e84dd40500","size":5,"soft_fail":false,"target_va":5369613520,"va":5369231486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t384669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e89dde0500","size":5,"soft_fail":false,"target_va":5369616080,"va":5369231406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd860
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t385048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e818e00500","size":5,"soft_fail":false,"target_va":5369616480,"va":5369231427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9950572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568176,"raw_hex":"e8942a68ff","size":5,"soft_fail":false,"target_va":5369617648,"va":5379568215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036f040
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3077483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e86bf52e00","size":5,"soft_fail":false,"target_va":5372309568,"va":5369232080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3517981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e81dae3500","size":5,"soft_fail":false,"target_va":5372749696,"va":5369231710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043be60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3917075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e813c53b00","size":5,"soft_fail":false,"target_va":5373148768,"va":5369231688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3927430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e886ed3b00","size":5,"soft_fail":false,"target_va":5373158800,"va":5369231365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3948226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e8c23e3c00","size":5,"soft_fail":false,"target_va":5373180176,"va":5369231945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3949520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e8d0433c00","size":5,"soft_fail":false,"target_va":5373181376,"va":5369231851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140445e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3957715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e8d3633c00","size":5,"soft_fail":false,"target_va":5373189648,"va":5369231928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5a660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568176,"raw_hex":"e898f3ffff","size":5,"soft_fail":false,"target_va":5379565152,"va":5379568323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33746110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e8beec0202","size":5,"soft_fail":false,"target_va":5402978144,"va":5369232029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33746917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e8e5ef0202","size":5,"soft_fail":false,"target_va":5402978912,"va":5369231990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23410048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379568176,"raw_hex":"e880356501","size":5,"soft_fail":false,"target_va":5402978880,"va":5379568827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3929421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e84df53b00","size":5,"soft_fail":false,"target_va":5373160864,"va":5369231438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::layout
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3929790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e8bef63b00","size":5,"soft_fail":false,"target_va":5373161712,"va":5369231917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::align_center
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3949713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e891443c00","size":5,"soft_fail":false,"target_va":5373181584,"va":5369231866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3949772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e8cc443c00","size":5,"soft_fail":false,"target_va":5373181680,"va":5369231903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::layout::LayoutStyle::xalign_center
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3949750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369231312,"raw_hex":"e8b6443c00","size":5,"soft_fail":false,"target_va":5373181632,"va":5369231877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5369231312,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5369231789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":9540,"parent_record_offset":9400,"ancestor_record_offset":9400,"inlinee":8022,"source_line":733,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":111284,"parent_record_offset":110964,"ancestor_record_offset":110964,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1018,"record_offset":112348,"parent_record_offset":110964,"ancestor_record_offset":110964,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":9400,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":9400,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":11016,"parent_record_offset":10980,"ancestor_record_offset":9400,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":58,"record_offset":11048,"parent_record_offset":11016,"ancestor_record_offset":9400,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":58,"record_offset":10980,"parent_record_offset":10860,"ancestor_record_offset":9400,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":58,"record_offset":10892,"parent_record_offset":10860,"ancestor_record_offset":9400,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":58,"record_offset":11180,"parent_record_offset":9664,"ancestor_record_offset":9400,"inlinee":8015,"source_line":263,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":58,"record_offset":11204,"parent_record_offset":11180,"ancestor_record_offset":9400,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":58,"record_offset":10500,"parent_record_offset":10476,"ancestor_record_offset":9400,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":58,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":9400,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":58,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":9400,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":58,"record_offset":9992,"parent_record_offset":9968,"ancestor_record_offset":9400,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":58,"record_offset":10164,"parent_record_offset":10140,"ancestor_record_offset":9400,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":58,"record_offset":11236,"parent_record_offset":9664,"ancestor_record_offset":9400,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":58,"record_offset":10828,"parent_record_offset":9664,"ancestor_record_offset":9400,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1018,"record_offset":112316,"parent_record_offset":111284,"ancestor_record_offset":110964,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":58,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":9400,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":58,"record_offset":10304,"parent_record_offset":10280,"ancestor_record_offset":9400,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":58,"record_offset":11124,"parent_record_offset":11096,"ancestor_record_offset":9400,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":58,"record_offset":10920,"parent_record_offset":10892,"ancestor_record_offset":9400,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":58,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":9400,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":58,"record_offset":10476,"parent_record_offset":10452,"ancestor_record_offset":9400,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":58,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":9400,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":58,"record_offset":9736,"parent_record_offset":9712,"ancestor_record_offset":9400,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":58,"record_offset":10452,"parent_record_offset":10232,"ancestor_record_offset":9400,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":58,"record_offset":9792,"parent_record_offset":9760,"ancestor_record_offset":9400,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":9400,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":10340,"parent_record_offset":10232,"ancestor_record_offset":9400,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":10280,"parent_record_offset":10256,"ancestor_record_offset":9400,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":10660,"parent_record_offset":9872,"ancestor_record_offset":9400,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":9400,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":9844,"parent_record_offset":9820,"ancestor_record_offset":9400,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":9820,"parent_record_offset":9760,"ancestor_record_offset":9400,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":9872,"parent_record_offset":9844,"ancestor_record_offset":9400,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":58,"record_offset":10112,"parent_record_offset":10084,"ancestor_record_offset":9400,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":58,"record_offset":10140,"parent_record_offset":10112,"ancestor_record_offset":9400,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":58,"record_offset":10200,"parent_record_offset":10084,"ancestor_record_offset":9400,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":58,"record_offset":10084,"parent_record_offset":9872,"ancestor_record_offset":9400,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":9916,"parent_record_offset":9872,"ancestor_record_offset":9400,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":58,"record_offset":10232,"parent_record_offset":9872,"ancestor_record_offset":9400,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":58,"record_offset":9944,"parent_record_offset":9872,"ancestor_record_offset":9400,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":58,"record_offset":10028,"parent_record_offset":9872,"ancestor_record_offset":9400,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":58,"record_offset":10568,"parent_record_offset":9872,"ancestor_record_offset":9400,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":58,"record_offset":9712,"parent_record_offset":9664,"ancestor_record_offset":9400,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":113432,"parent_record_offset":112348,"ancestor_record_offset":110964,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":9608,"parent_record_offset":9540,"ancestor_record_offset":9400,"inlinee":5592,"source_line":749,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::get_content_mut_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":9664,"parent_record_offset":9540,"ancestor_record_offset":9400,"inlinee":8013,"source_line":867,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":112376,"parent_record_offset":112348,"ancestor_record_offset":110964,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1018,"record_offset":111164,"parent_record_offset":111140,"ancestor_record_offset":110964,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/01018.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$15::get_content_mut_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":10860,"parent_record_offset":10828,"ancestor_record_offset":9400,"inlinee":8017,"source_line":876,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":9400,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":9580,"parent_record_offset":9540,"ancestor_record_offset":9400,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":58,"record_offset":9632,"parent_record_offset":9608,"ancestor_record_offset":9400,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$18::pill_button::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":58,"record_offset":11304,"parent_record_offset":9540,"ancestor_record_offset":9400,"inlinee":8026,"source_line":619,"archive":"_pdb/A/modules/00058.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$18::pill_button::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 619
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F5A
// 首条位置: {"module":58,"inlinee":8026}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::back_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 636
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":11472,"rva":4439040,"va":5373148160}
// 编译实例 RVAs: 0x43bc00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3534708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148160,"raw_hex":"e88c10caff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373148223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5b230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6419995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148160,"raw_hex":"e81bf66100","size":5,"soft_fail":false,"target_va":5379568176,"va":5373148176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29830204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148160,"raw_hex":"e83c2cc701","size":5,"soft_fail":false,"target_va":5402978880,"va":5373148671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::mute_click
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":318,"record_offset":11596,"parent_record_offset":11472,"ancestor_record_offset":11472,"inlinee":17210,"source_line":648,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":318,"record_offset":12748,"parent_record_offset":11472,"ancestor_record_offset":11472,"inlinee":17211,"source_line":660,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":11472,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":11932,"parent_record_offset":11908,"ancestor_record_offset":11472,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":11472,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":11472,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":12128,"parent_record_offset":12104,"ancestor_record_offset":11472,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":318,"record_offset":12680,"parent_record_offset":11652,"ancestor_record_offset":11472,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":12376,"parent_record_offset":12352,"ancestor_record_offset":11472,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":11472,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":12440,"parent_record_offset":12416,"ancestor_record_offset":11472,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":11728,"parent_record_offset":11704,"ancestor_record_offset":11472,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":11472,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":12416,"parent_record_offset":12196,"ancestor_record_offset":11472,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":11760,"parent_record_offset":11728,"ancestor_record_offset":11472,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12328,"parent_record_offset":12304,"ancestor_record_offset":11472,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12304,"parent_record_offset":12196,"ancestor_record_offset":11472,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":11472,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12624,"parent_record_offset":11836,"ancestor_record_offset":11472,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":11472,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":11812,"parent_record_offset":11788,"ancestor_record_offset":11472,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":11788,"parent_record_offset":11728,"ancestor_record_offset":11472,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":11472,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":12076,"parent_record_offset":12048,"ancestor_record_offset":11472,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":11472,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":12164,"parent_record_offset":12048,"ancestor_record_offset":11472,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":12048,"parent_record_offset":11836,"ancestor_record_offset":11472,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":11880,"parent_record_offset":11836,"ancestor_record_offset":11472,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12196,"parent_record_offset":11836,"ancestor_record_offset":11472,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":11908,"parent_record_offset":11836,"ancestor_record_offset":11472,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":11992,"parent_record_offset":11836,"ancestor_record_offset":11472,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":12532,"parent_record_offset":11836,"ancestor_record_offset":11472,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":11680,"parent_record_offset":11652,"ancestor_record_offset":11472,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":12712,"parent_record_offset":11624,"ancestor_record_offset":11472,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":12776,"parent_record_offset":12748,"ancestor_record_offset":11472,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":11652,"parent_record_offset":11624,"ancestor_record_offset":11472,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":318,"record_offset":11624,"parent_record_offset":11596,"ancestor_record_offset":11472,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":12488,"parent_record_offset":12464,"ancestor_record_offset":11472,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::PillButtonView::on_click
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 644
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14692
// 首条位置: {"module":2468,"inlinee":83602}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":58788,"parent_record_offset":58764,"ancestor_record_offset":58764,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2468,"record_offset":59620,"parent_record_offset":59596,"ancestor_record_offset":58764,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":59088,"parent_record_offset":59064,"ancestor_record_offset":58764,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2468,"record_offset":59172,"parent_record_offset":59148,"ancestor_record_offset":58764,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2468,"record_offset":59112,"parent_record_offset":59088,"ancestor_record_offset":58764,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2468,"record_offset":59284,"parent_record_offset":59260,"ancestor_record_offset":58764,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":59532,"parent_record_offset":59508,"ancestor_record_offset":58764,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2468,"record_offset":59424,"parent_record_offset":59400,"ancestor_record_offset":58764,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2468,"record_offset":59596,"parent_record_offset":59572,"ancestor_record_offset":58764,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":58884,"parent_record_offset":58860,"ancestor_record_offset":58764,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":58860,"parent_record_offset":58836,"ancestor_record_offset":58764,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":59572,"parent_record_offset":59352,"ancestor_record_offset":58764,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2468,"record_offset":58916,"parent_record_offset":58884,"ancestor_record_offset":58764,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":59484,"parent_record_offset":59460,"ancestor_record_offset":58764,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":59460,"parent_record_offset":59352,"ancestor_record_offset":58764,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":59400,"parent_record_offset":59376,"ancestor_record_offset":58764,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":59780,"parent_record_offset":58992,"ancestor_record_offset":58764,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":59376,"parent_record_offset":59352,"ancestor_record_offset":58764,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":58968,"parent_record_offset":58944,"ancestor_record_offset":58764,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":58944,"parent_record_offset":58884,"ancestor_record_offset":58764,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":58992,"parent_record_offset":58968,"ancestor_record_offset":58764,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":59232,"parent_record_offset":59204,"ancestor_record_offset":58764,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":59260,"parent_record_offset":59232,"ancestor_record_offset":58764,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":59320,"parent_record_offset":59204,"ancestor_record_offset":58764,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2468,"record_offset":59204,"parent_record_offset":58992,"ancestor_record_offset":58764,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":59036,"parent_record_offset":58992,"ancestor_record_offset":58764,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2468,"record_offset":59352,"parent_record_offset":58992,"ancestor_record_offset":58764,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":59064,"parent_record_offset":58992,"ancestor_record_offset":58764,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":59148,"parent_record_offset":58992,"ancestor_record_offset":58764,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2468,"record_offset":59688,"parent_record_offset":58992,"ancestor_record_offset":58764,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2468,"record_offset":58836,"parent_record_offset":58812,"ancestor_record_offset":58764,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":59840,"parent_record_offset":58788,"ancestor_record_offset":58764,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":58812,"parent_record_offset":58788,"ancestor_record_offset":58764,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":59644,"parent_record_offset":59620,"ancestor_record_offset":58764,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::PillButtonView::mute_click
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 648
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x433A
// 首条位置: {"module":318,"inlinee":17210}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":318,"record_offset":11624,"parent_record_offset":11596,"ancestor_record_offset":11596,"inlinee":17192,"source_line":110,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":11596,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":11932,"parent_record_offset":11908,"ancestor_record_offset":11596,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":11596,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":11596,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":12128,"parent_record_offset":12104,"ancestor_record_offset":11596,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":318,"record_offset":12680,"parent_record_offset":11652,"ancestor_record_offset":11596,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":12376,"parent_record_offset":12352,"ancestor_record_offset":11596,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":11596,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":12440,"parent_record_offset":12416,"ancestor_record_offset":11596,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":11728,"parent_record_offset":11704,"ancestor_record_offset":11596,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":11596,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":12416,"parent_record_offset":12196,"ancestor_record_offset":11596,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":11760,"parent_record_offset":11728,"ancestor_record_offset":11596,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12328,"parent_record_offset":12304,"ancestor_record_offset":11596,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12304,"parent_record_offset":12196,"ancestor_record_offset":11596,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":11596,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12624,"parent_record_offset":11836,"ancestor_record_offset":11596,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":11596,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":11812,"parent_record_offset":11788,"ancestor_record_offset":11596,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":11788,"parent_record_offset":11728,"ancestor_record_offset":11596,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":11596,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":12076,"parent_record_offset":12048,"ancestor_record_offset":11596,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":11596,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":12164,"parent_record_offset":12048,"ancestor_record_offset":11596,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":12048,"parent_record_offset":11836,"ancestor_record_offset":11596,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":11880,"parent_record_offset":11836,"ancestor_record_offset":11596,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12196,"parent_record_offset":11836,"ancestor_record_offset":11596,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":11908,"parent_record_offset":11836,"ancestor_record_offset":11596,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":11992,"parent_record_offset":11836,"ancestor_record_offset":11596,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":12532,"parent_record_offset":11836,"ancestor_record_offset":11596,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":11680,"parent_record_offset":11652,"ancestor_record_offset":11596,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":12712,"parent_record_offset":11624,"ancestor_record_offset":11596,"inlinee":17193,"source_line":221,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":11652,"parent_record_offset":11624,"ancestor_record_offset":11596,"inlinee":17191,"source_line":938,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":12488,"parent_record_offset":12464,"ancestor_record_offset":11596,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::PillButtonView::inactive_opacity
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 655
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4370
// 首条位置: {"module":318,"record_offset":13840,"rva":4439584,"va":5373148704}
// 编译实例 RVAs: 0x43be20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5a660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6416414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148704,"raw_hex":"e81ee86100","size":5,"soft_fail":false,"target_va":5379565152,"va":5373148733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":13980,"parent_record_offset":13840,"ancestor_record_offset":13840,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::PillButtonView::size
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 660
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x433B
// 首条位置: {"module":318,"inlinee":17211}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":12776,"parent_record_offset":12748,"ancestor_record_offset":12748,"inlinee":17212,"source_line":827,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":318,"record_offset":13584,"parent_record_offset":13560,"ancestor_record_offset":12748,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":13052,"parent_record_offset":13028,"ancestor_record_offset":12748,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":318,"record_offset":13136,"parent_record_offset":13112,"ancestor_record_offset":12748,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":318,"record_offset":13076,"parent_record_offset":13052,"ancestor_record_offset":12748,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":318,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":12748,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":318,"record_offset":13800,"parent_record_offset":12776,"ancestor_record_offset":12748,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":13496,"parent_record_offset":13472,"ancestor_record_offset":12748,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":318,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":12748,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":318,"record_offset":13560,"parent_record_offset":13536,"ancestor_record_offset":12748,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":12852,"parent_record_offset":12828,"ancestor_record_offset":12748,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":12828,"parent_record_offset":12804,"ancestor_record_offset":12748,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":13536,"parent_record_offset":13316,"ancestor_record_offset":12748,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":318,"record_offset":12880,"parent_record_offset":12852,"ancestor_record_offset":12748,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":12748,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":13424,"parent_record_offset":13316,"ancestor_record_offset":12748,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":12748,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":13744,"parent_record_offset":12956,"ancestor_record_offset":12748,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":12748,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":12748,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12908,"parent_record_offset":12852,"ancestor_record_offset":12748,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":12748,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":13196,"parent_record_offset":13168,"ancestor_record_offset":12748,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":13224,"parent_record_offset":13196,"ancestor_record_offset":12748,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":13284,"parent_record_offset":13168,"ancestor_record_offset":12748,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":318,"record_offset":13168,"parent_record_offset":12956,"ancestor_record_offset":12748,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":13000,"parent_record_offset":12956,"ancestor_record_offset":12748,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":318,"record_offset":13316,"parent_record_offset":12956,"ancestor_record_offset":12748,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":13028,"parent_record_offset":12956,"ancestor_record_offset":12748,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":13112,"parent_record_offset":12956,"ancestor_record_offset":12748,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":318,"record_offset":13652,"parent_record_offset":12956,"ancestor_record_offset":12748,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":318,"record_offset":12804,"parent_record_offset":12776,"ancestor_record_offset":12748,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$43::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":12748,"inlinee":5642,"source_line":58,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$43::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 675
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x433E
// 首条位置: {"module":318,"inlinee":17214}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$20::create_node_content
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 686
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4372
// 首条位置: {"module":318,"record_offset":14012,"rva":4439648,"va":5373148768}
// 编译实例 RVAs: 0x43be60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1491965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148768,"raw_hex":"e8033ce9ff","size":5,"soft_fail":false,"target_va":5371656896,"va":5373148856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":14176,"parent_record_offset":14012,"ancestor_record_offset":14012,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$43::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":318,"record_offset":14152,"parent_record_offset":14012,"ancestor_record_offset":14012,"inlinee":17214,"source_line":675,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":318,"record_offset":14216,"parent_record_offset":14196,"ancestor_record_offset":14012,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":14196,"parent_record_offset":14176,"ancestor_record_offset":14012,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":318,"record_offset":14248,"parent_record_offset":14216,"ancestor_record_offset":14012,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":318,"record_offset":14272,"parent_record_offset":14248,"ancestor_record_offset":14012,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":318,"record_offset":14304,"parent_record_offset":14216,"ancestor_record_offset":14012,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$21::prepare
// 记录形式: compiled_procedure; instances=1
// 源码行: 692
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":14348,"rva":4439760,"va":5373148880}
// 编译实例 RVAs: 0x43bed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-399413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148880,"raw_hex":"e8cbe7f9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373149120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056c370
//     owner=unresolved_namespace; count=2
//     target_aliases=18 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1246298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148880,"raw_hex":"e85a041300","size":5,"soft_fail":false,"target_va":5374395248,"va":5373148945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407bf410
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3683590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148880,"raw_hex":"e806353800","size":5,"soft_fail":false,"target_va":5376832528,"va":5373148933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a624b0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6447908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148880,"raw_hex":"e824636200","size":5,"soft_fail":false,"target_va":5379597488,"va":5373149575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a68060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: widgets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6471757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148880,"raw_hex":"e84dc06200","size":5,"soft_fail":false,"target_va":5379620960,"va":5373149198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6470764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148880,"raw_hex":"e86cbc6200","size":5,"soft_fail":false,"target_va":5379619792,"va":5373149023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::corner_radius
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6470438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148880,"raw_hex":"e826bb6200","size":5,"soft_fail":false,"target_va":5379619440,"va":5373148997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::fill_color
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6470281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373148880,"raw_hex":"e889ba6200","size":5,"soft_fail":false,"target_va":5379619264,"va":5373148978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":318,"record_offset":15028,"parent_record_offset":14348,"ancestor_record_offset":14348,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":14480,"parent_record_offset":14348,"ancestor_record_offset":14348,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$17::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":15060,"parent_record_offset":14348,"ancestor_record_offset":14348,"inlinee":17206,"source_line":977,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":14348,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":318,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":14348,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":318,"record_offset":14892,"parent_record_offset":14864,"ancestor_record_offset":14348,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":318,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":14348,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":318,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":14348,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":318,"record_offset":14920,"parent_record_offset":14892,"ancestor_record_offset":14348,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":318,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":14348,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::mei::widgets::impl$21::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 717
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":318,"record_offset":15640,"rva":4440528,"va":5373149648}
// 编译实例 RVAs: 0x43c1d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::rect
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2887716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373149648,"raw_hex":"e824102c00","size":5,"soft_fail":false,"target_va":5376037504,"va":5373149783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":15944,"parent_record_offset":15640,"ancestor_record_offset":15640,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":15860,"parent_record_offset":15640,"ancestor_record_offset":15640,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":15776,"parent_record_offset":15640,"ancestor_record_offset":15640,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::RenderContext::draw_rect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":318,"record_offset":16028,"parent_record_offset":15640,"ancestor_record_offset":15640,"inlinee":17200,"source_line":1035,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":318,"record_offset":16056,"parent_record_offset":16028,"ancestor_record_offset":15640,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":15800,"parent_record_offset":15776,"ancestor_record_offset":15640,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":15968,"parent_record_offset":15944,"ancestor_record_offset":15640,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":318,"record_offset":15884,"parent_record_offset":15860,"ancestor_record_offset":15640,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":16112,"parent_record_offset":16028,"ancestor_record_offset":15640,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00318.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":318,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":15640,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/00318.jsonl.gz"}
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
// 完整结构及原始类型记录: widgets.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::FrameContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::FrameContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::mei::widgets::FrameContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::mei::widgets::FrameContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::mei::widgets::FrameContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::mei::widgets::FrameContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::mei::widgets::FrameContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::on_input<toolbar::mei::widgets::FrameContent>", "constructor": "toolbar::mei::core::NodeContent::on_input", "arguments": ["toolbar::mei::widgets::FrameContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::mei::widgets::FrameContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::mei::widgets::FrameContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::mei::widgets::FrameContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::mei::widgets::FrameContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::mei::widgets::FrameContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::mei::widgets::FrameContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::FrameView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::ImageContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 22
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::mei::widgets::ImageContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::mei::widgets::ImageContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::mei::widgets::ImageContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::mei::widgets::ImageContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::mei::widgets::ImageContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::mei::widgets::ImageContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::mei::widgets::ImageContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::mei::widgets::ImageContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::mei::widgets::ImageContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::mei::widgets::ImageContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::on_input<toolbar::mei::widgets::ImageContent>", "constructor": "toolbar::mei::core::NodeContent::on_input", "arguments": ["toolbar::mei::widgets::ImageContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::ImageContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::mei::widgets::ImageContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::mei::widgets::ImageContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::mei::widgets::ImageContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::ImageView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::LabelContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content_mut<toolbar::mei::widgets::LabelContent>", "constructor": "toolbar::mei::core::NodeContext::content_mut", "arguments": ["toolbar::mei::widgets::LabelContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::mei::widgets::LabelContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::mei::widgets::LabelContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::mei::widgets::LabelContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::mei::widgets::LabelContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::mei::widgets::LabelContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::mei::widgets::LabelContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::mei::widgets::LabelContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::mei::widgets::LabelContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::mei::widgets::LabelContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::mei::widgets::LabelContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::mei::widgets::LabelContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::LabelView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::LabelView::metrics<f32>", "constructor": "toolbar::mei::widgets::LabelView::metrics", "arguments": ["f32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::MenuButtonContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content_mut<toolbar::mei::widgets::MenuButtonContent>", "constructor": "toolbar::mei::core::NodeContext::content_mut", "arguments": ["toolbar::mei::widgets::MenuButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::MenuButtonContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::mei::widgets::MenuButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::mei::widgets::MenuButtonContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::mei::widgets::MenuButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::mei::widgets::MenuButtonContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::mei::widgets::MenuButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::on_input<toolbar::mei::widgets::MenuButtonContent>", "constructor": "toolbar::mei::core::NodeContent::on_input", "arguments": ["toolbar::mei::widgets::MenuButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::mei::widgets::MenuButtonContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::mei::widgets::MenuButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::mei::widgets::MenuButtonContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::mei::widgets::MenuButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::mei::widgets::MenuButtonContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::mei::widgets::MenuButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::MenuButtonView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 13; 过程引用数: 7
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::PillButtonContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::mei::widgets::PillButtonContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::mei::widgets::PillButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any_mut<toolbar::mei::widgets::PillButtonContent>", "constructor": "toolbar::mei::core::impl$18::as_any_mut", "arguments": ["toolbar::mei::widgets::PillButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::as_any<toolbar::mei::widgets::PillButtonContent>", "constructor": "toolbar::mei::core::impl$18::as_any", "arguments": ["toolbar::mei::widgets::PillButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$18::type_name<toolbar::mei::widgets::PillButtonContent>", "constructor": "toolbar::mei::core::impl$18::type_name", "arguments": ["toolbar::mei::widgets::PillButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::mei::widgets::PillButtonContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::mei::widgets::PillButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::PillButtonContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::mei::widgets::PillButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::on_input<toolbar::mei::widgets::PillButtonContent>", "constructor": "toolbar::mei::core::NodeContent::on_input", "arguments": ["toolbar::mei::widgets::PillButtonContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::mei::widgets::PillButtonView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: widgets.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: widgets.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
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

// ------------------------------------------------------------------
// 函数: toolbar::mei::core::NodeContext::sounds
// 信息 ID: 95d27ab33e1ad2084b940cfa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> > @ 0x7fa49 -> 0x443910 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::widgets::MenuButtonView::on_click; module=318, inline_record=4496; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::back_button; module=318, inline_record=11624; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::menu_button; module=319, inline_record=72808; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::menu_button_with_id; module=319, inline_record=74216; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::report_bug::impl$8::toggle_icon; module=319, inline_record=77952; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> >; module=1018, inline_record=112348; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=58788; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=61356; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::FrameView::draw
// 信息 ID: f23df9cfcb29a8f02c76467c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::instantiate_ui_template_impl @ 0xa68a57 -> 0x43ba10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::ImageView::blur_extend
// 信息 ID: 16d9e7a1b2df91f37a91d888
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x2d12d3 -> 0x43b680 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::new_session::ui; module=2499, inline_record=21396; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::new_session::ui; module=2499, inline_record=23992; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::ImageView::tint
// 信息 ID: 6ace761e225e42b17c9ebf0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x2d12cb -> 0x43b640 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::new_session::ui; module=2499, inline_record=23896; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::LabelView::align
// 信息 ID: fa902d2eb484cd9670fc0325
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dba5 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1ba -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e7fc -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eda3 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f333 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f862 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fda4 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x20303 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x20846 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20d8a -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> > @ 0x21315 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game::closure$0 @ 0x33f451 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game::closure$0 @ 0x33f4b5 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game::closure$0 @ 0x33f511 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::new_session::ui @ 0x16662b7 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_display_system::hint_draw::closure_env$0,ref$<str$> > @ 0x1aa60c8 -> 0x43b3c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=8348; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$6,ref$<str$> >; module=2468, inline_record=32824; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$6,ref$<str$> >; module=2468, inline_record=33820; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::daily_welcome::ui::closure$3; module=2468, inline_record=106388; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::daily_welcome::ui::closure$3; module=2468, inline_record=106892; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::new_session::ui::closure$1; module=2468, inline_record=111384; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::report_bug::ui::closure$6; module=2468, inline_record=122904; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=7948; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::LabelView::color
// 信息 ID: 694b6aaea09da47dcd91fd11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 28 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9d1 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1db9a -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfb8 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1af -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5a8 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e7f1 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebef -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ed98 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f17f -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f328 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f70a -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f857 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbab -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fd99 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x20183 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x202f8 -> 0x43b480 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=77568; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=79068; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=80540; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=8072; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2421, inline_record=7232; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::report_bug::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=55500; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::daily_welcome::ui::closure$0; module=2468, inline_record=103252; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::daily_welcome::ui::closure$3; module=2468, inline_record=106208; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::LabelView::family
// 信息 ID: 8b77d89c21f5b8f6f4a1197b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 29 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9a9 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1db8f -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1df90 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1a4 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e580 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e7e6 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebc7 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ed8d -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f157 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f31d -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f6e2 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f84c -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fb83 -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fd8e -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2015b -> 0x43b400 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x202ed -> 0x43b400 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=77508; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=79008; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=80480; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=8284; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2421, inline_record=7172; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$6,ref$<str$> >; module=2468, inline_record=32944; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$6,ref$<str$> >; module=2468, inline_record=33940; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::daily_welcome::ui::closure$3; module=2468, inline_record=106448; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::LabelView::max_width
// 信息 ID: 082b3a24d3d94f25bf1a0582
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 13 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9b9 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfa0 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e590 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebd7 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f167 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f6f2 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fb93 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2016b -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x20650 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20c18 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> > @ 0x210c3 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$5,ref$<str$> > @ 0x33e0b0 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::new_session::ui @ 0x16662e2 -> 0x43b370 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=8192; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$6,ref$<str$> >; module=2468, inline_record=33004; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$6,ref$<str$> >; module=2468, inline_record=34000; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::new_session::ui::closure$1; module=2468, inline_record=111320; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::report_bug::ui::closure$6; module=2468, inline_record=122844; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=3196; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::LabelView::metrics
// 信息 ID: 894d999dfcddb3395892d03a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 67 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=15, inline_record=2024; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=15, inline_record=2708; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=4836; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=5504; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=7524; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=8288; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=10372; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=11028; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::LabelView::metrics<f32>
// 信息 ID: 7f7d50c8aa7e24e9b3168fc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::LabelView::metrics -> ["f32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::new_session::ui @ 0x16662c7 -> 0x15ed080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::LabelView::weight
// 信息 ID: dc98685093de81c4290bcce1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9c6 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfad -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e59d -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebe4 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f174 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f6ff -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fba0 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x20178 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x2065d -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20c25 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> > @ 0x210d0 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$1,ref$<str$> > @ 0x33ca0f -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$1,ref$<str$> > @ 0x33cb3b -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4 @ 0x3413e1 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<system_onboarding::hint_display_system::hint_draw::closure_env$0,ref$<str$> > @ 0x1aa6092 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_draw::closure$1::closure$1 @ 0x1aa6d68 -> 0x43b440 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=80420; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=8436; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=16928; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=18668; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$3,ref$<str$> >; module=2468, inline_record=25528; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$3,ref$<str$> >; module=2468, inline_record=26404; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=47104; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=48844; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::MenuButtonView::align
// 信息 ID: 240d4daa43162204e8fba7da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x2842a4 -> 0x43b330 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$3 @ 0x340b37 -> 0x43b330 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::new_session::ui; module=2499, inline_record=11268; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::MenuButtonView::color
// 信息 ID: ad12d3e0cf86d5481898a03e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0> @ 0x7f38a -> 0x43b560 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::new_session::ui; module=2499, inline_record=11148; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::MenuButtonView::force_highlight
// 信息 ID: e55f5be7a3f1474aa9e28c44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$3 @ 0x340b52 -> 0x43b5a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::MenuButtonView::into_inner
// 信息 ID: b30705d0aebf71450d5e804c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 179 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9d9 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dbad -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfc0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1c2 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5b0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e804 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebf7 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1edab -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f187 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f33b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f712 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f86a -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbb3 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fdac -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2018b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2030b -> 0x43c290 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::MenuButtonView::max_width
// 信息 ID: 857fbd6021fca47e59721b08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x284299 -> 0x43b2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls @ 0x2f0b16 -> 0x43b2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$3 @ 0x340b2c -> 0x43b2e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::new_session::ui; module=2499, inline_record=11208; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::MenuButtonView::metrics
// 信息 ID: 468a7c3bfb3ac720093a3146
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=15, inline_record=2172; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> >; module=15, inline_record=3836; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=4984; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > >; module=15, inline_record=6632; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=7672; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> >; module=15, inline_record=9416; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=10520; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >; module=15, inline_record=12156; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::MenuButtonView::on_click
// 信息 ID: 84df419486111909bc89304f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 29 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1da9c -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dcab -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e088 -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e2cb -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e678 -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e90b -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ecbb -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eeab -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f24b -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f43b -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f7d7 -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f96b -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fc7b -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1feab -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x20250 -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2040b -> 0x43b1a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$4,ref$<str$> >; module=2468, inline_record=95084; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$4,ref$<str$> >; module=2468, inline_record=97648; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::new_session::ui; module=2499, inline_record=11328; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=4576; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>; module=3685, inline_record=9096; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::MenuButtonView::weight
// 信息 ID: cff585b8e4e57c3acab25231
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::NodeContext::with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0> @ 0x7f37c -> 0x43b2a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::PillButtonView::inactive_opacity
// 信息 ID: 115f59d8a07a36e94bd929c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1 @ 0x33eaed -> 0x43be20 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1 @ 0x33eb90 -> 0x43be20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::load::delete_button::closure$0; module=2468, inline_record=108316; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::PillButtonView::mute_click
// 信息 ID: 03f345c91231bb74fd337592
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::back_button; module=318, inline_record=11596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::PillButtonView::on_click
// 信息 ID: f3d2b3af360de2e77b2c5713
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=58764; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=61332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::PillButtonView::size
// 信息 ID: c53b8cf817e492e357e19d8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::back_button; module=318, inline_record=12748; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::load::delete_button::closure$0; module=2468, inline_record=108376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::back_button
// 信息 ID: a97b9601e5d1adcd74ecc44a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2,ref$<str$> > @ 0x33b05a -> 0x43bc00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$16,ref$<str$> > @ 0x33b40a -> 0x43bc00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1 @ 0x33eae2 -> 0x43bc00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1 @ 0x33eb85 -> 0x43bc00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::photomode::lut_gallery::ui::closure_env$0,ref$<str$> > @ 0x1619cab -> 0x43bc00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::photomode::lut_gallery::ui::closure_env$0,ref$<str$> > @ 0x1619e74 -> 0x43bc00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$0,ref$<str$> > @ 0x161c60e -> 0x43bc00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$0,ref$<str$> > @ 0x161c8c4 -> 0x43bc00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::frame
// 信息 ID: df97caef5210179fddb49941
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=1876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::image::closure$0
// 信息 ID: e45528e80472d5d38fc1cfd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::mei::widgets::impl$0::image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::image<ref$<str$> >; module=58, inline_record=7448; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::image<u64>; module=58, inline_record=8548; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::image<ref$<str$> >; module=1018, inline_record=110112; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::image<ref$<str$> >; module=3457, inline_record=4376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::image<ref$<str$> >
// 信息 ID: 3ca08c6228dd15e81a326c28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::impl$0::image -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 24 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x2841db -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings @ 0x2d12bd -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_controls::show_gui_controls::closure_env$0,ref$<str$> > @ 0x33a23d -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10,ref$<str$> > @ 0x33a945 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$14,ref$<str$> > @ 0x33acd5 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> > @ 0x33c136 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> > @ 0x33c8a6 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> > @ 0x33d17e -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> > @ 0x33d946 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x340175 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x340461 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$1 @ 0x340873 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$17 @ 0x343719 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$4::closure$21 @ 0x343e17 -> 0x7f550 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$0::decorate_hovered @ 0x43af0f -> 0xa5b0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$0::decorate_hovered @ 0x43b085 -> 0xa5b0e0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::image<u64>
// 信息 ID: 836e9b7f34cccfbd2eb3dffd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::mei::widgets::impl$0::image -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33f77d -> 0x7f690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::label
// 信息 ID: 97ade1bd12703f88cf5e9eca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 49 条；内联宿主 43 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d97d -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1db63 -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1df5b -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e173 -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e54b -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e7b5 -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eb92 -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ed5c -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f122 -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f2ec -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f6ad -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f81b -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fb4e -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fd5d -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x20126 -> 0x443570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x202bc -> 0x443570 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=7668; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2421, inline_record=6912; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$5,ref$<str$> >; module=2468, inline_record=14948; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=16664; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=18404; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$3,ref$<str$> >; module=2468, inline_record=25268; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$3,ref$<str$> >; module=2468, inline_record=26144; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$6,ref$<str$> >; module=2468, inline_record=32624; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::label::closure$0
// 信息 ID: 43578f1fcf22b436599277e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::mei::widgets::impl$0::label", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::label; module=319, inline_record=72460; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=1025, inline_record=7836; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2421, inline_record=7076; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$5,ref$<str$> >; module=2468, inline_record=15092; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=16832; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=18572; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$3,ref$<str$> >; module=2468, inline_record=25432; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$3,ref$<str$> >; module=2468, inline_record=26308; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::label_with_id
// 信息 ID: 656fd2e6e9494a3712e46067
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=77276; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=78776; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=80188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::label_with_id::closure$0
// 信息 ID: b593695f1491349991b74725
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::mei::widgets::impl$0::label_with_id", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=77412; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=78912; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=80324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::menu_button
// 信息 ID: 6924fab09f68f279b373efff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 66 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1da70 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dca0 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e05b -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e2c0 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e64b -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e900 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ec8e -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eea0 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f21e -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f430 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f7aa -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f960 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fc4e -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fea0 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x20223 -> 0x443620 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x20400 -> 0x443620 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::menu_button::closure$0
// 信息 ID: 53c35377a0de615c80953eaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::mei::widgets::impl$0::menu_button", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::menu_button; module=319, inline_record=72776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::menu_button_with_id
// 信息 ID: a0a4ba699b714d45d2b3115c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls::closure$1 @ 0x33e685 -> 0x4437a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_controls::show_gui_controls::closure$1 @ 0x33e430 -> 0x4437a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$0::menu_button_with_id::closure$0
// 信息 ID: 023d759c70638c75fffec56f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::mei::widgets::impl$0::menu_button_with_id", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::impl$0::menu_button_with_id; module=319, inline_record=74184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$10::create_node_content
// 信息 ID: 4faa1ee928732383e9e69f68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::ImageContent> @ 0x7f1c8 -> 0x43b6b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$0::image<ref$<str$> > @ 0x1aa5d28 -> 0x43b6b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::ImageContent>; module=1018, inline_record=109264; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$11::prepare
// 信息 ID: af8b6aa8086ffd4de93e260a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$11::render
// 信息 ID: 93b25f35221446dbf0215f1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$13::create_node_content
// 信息 ID: f604ed7ba502f42684e7a2d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::FrameContent>; module=1018, inline_record=101040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$14::interactive
// 信息 ID: 63618f6beddb05e4804fb428
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer> @ 0x80be3 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer> @ 0x80c7a -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::impl$0::add_decorator @ 0x503eab -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a39f -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a3aa -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0ac86 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d9641 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d9665 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::create_roof::create_roof @ 0x1988b07 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::set_duck_velocities_towards_goals @ 0x1b34118 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43cc3 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6de5b -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6de63 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorators_on_glade_unload @ 0x1c6ee51 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3 @ 0x1e6ab84 -> 0x3390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$14::prepare
// 信息 ID: 7ce9ad317824804c115ddfbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$14::render
// 信息 ID: 278447a9ec12e34dddeb79cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$18::pill_button::closure$0
// 信息 ID: c6c59d911ec274816ccdfa1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::mei::widgets::impl$18::pill_button", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> >; module=58, inline_record=11304; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> >; module=1018, inline_record=111252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$18::pill_button<ref$<str$> >
// 信息 ID: 1530f1e93fbc4727ad1e674e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] toolbar::mei::widgets::impl$18::pill_button -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8 @ 0x33ef6c -> 0x7f7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8 @ 0x33efd5 -> 0x7f7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8 @ 0x33f049 -> 0x7f7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::back_button @ 0x43bc10 -> 0xa5b230 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::delete_button::closure$0 @ 0x162033b -> 0xa5b230 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::new_session::ui::closure$2 @ 0x1620850 -> 0xa5b230 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::new_session::ui::closure$2 @ 0x162096a -> 0xa5b230 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::new_session::ui::closure$2 @ 0x1620a7b -> 0xa5b230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$20::create_node_content
// 信息 ID: f6eb16a5838b100a408c751e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$18::pill_button<ref$<str$> > @ 0x7f948 -> 0x43be60 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::PillButtonContent>; module=1018, inline_record=103144; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$21::prepare
// 信息 ID: b6434cf10a7809a52278f0cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$21::render
// 信息 ID: 8a16a40da50eab732654c5f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$23::deref
// 信息 ID: e19b5d4fdfee56350717d018
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=80656; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=81884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$24::deref_mut
// 信息 ID: 02c894c82acbf418389c818e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1018, inline_record=77628; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2421, inline_record=7292; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=16988; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$1,ref$<str$> >; module=2468, inline_record=18728; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=47164; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=2468, inline_record=48904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$26::deref
// 信息 ID: eb5cc9f3dcc1482cef7a5c47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::decorate::impl$0::decorate_hovered; module=318, inline_record=548; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::report_bug::ui::closure$12::closure_env$0>; module=1018, inline_record=55128; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::report_bug::ui::closure$12::closure_env$0>; module=1018, inline_record=56288; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::report_bug::ui::closure$12::closure_env$0>; module=1018, inline_record=57680; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; module=1018, inline_record=67444; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; module=1018, inline_record=69460; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; module=1018, inline_record=70908; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2428, inline_record=5732; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$30::deref_mut
// 信息 ID: 8882312bbc2d4b0730c89a4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0>; module=1018, inline_record=60288; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0>; module=1018, inline_record=62588; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0>; module=1018, inline_record=64888; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0>; module=1018, inline_record=66024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$32::deref
// 信息 ID: 5fc94710add17d1053c5776a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 70 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d946 -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1daad -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dcd8 -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1df2b -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e099 -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e2fd -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e51b -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e689 -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e93d -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eb5b -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eccc -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eedd -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f0eb -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f25c -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f46d -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f677 -> 0x43c2a0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$33::deref_mut
// 信息 ID: cb59ccc9ed0a049947632e4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33f739 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33f791 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33fa72 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33fa84 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33fa94 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x340189 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x3403f7 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x340475 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$1 @ 0x340510 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$1 @ 0x340525 -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$1 @ 0x34064f -> 0x43c390 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$1 @ 0x340887 -> 0x43c390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$38::default
// 信息 ID: 6081efc4a4b685360b8ce288
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::FrameContent>; module=1018, inline_record=101068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$39::default
// 信息 ID: 4213e1dccc8f2836d1d84b9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$40::deref
// 信息 ID: 105efa682d17763c9950da22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::photomode::lut_gallery::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=28356; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::photomode::lut_gallery::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=29804; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=59876; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::new_session::ui::closure_env$0,ref$<str$> >; module=2468, inline_record=62444; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::load::delete_button::closure$0; module=2468, inline_record=109476; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::new_session::ui::closure$2; module=2468, inline_record=112044; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::new_session::ui::closure$2; module=2468, inline_record=113264; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::new_session::ui::closure$2; module=2468, inline_record=114484; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$43::default
// 信息 ID: e21571a5cfb44b9c45689820
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::impl$20::create_node_content; module=318, inline_record=14152; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::PillButtonContent>; module=1018, inline_record=103172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$4::create_node_content
// 信息 ID: 97087b9e72a2e9399151e0e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>; module=1018, inline_record=97212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$5::prepare
// 信息 ID: 6b56262eae7b8d19c92b387d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$5::render
// 信息 ID: a35ad10be1c20e17c8652145
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$7::create_node_content
// 信息 ID: 091682d9c1089962e476cba6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::MenuButtonContent>; module=1018, inline_record=107324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::mei::widgets::impl$8::render
// 信息 ID: a42be5a3067e951d1e69bd59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
