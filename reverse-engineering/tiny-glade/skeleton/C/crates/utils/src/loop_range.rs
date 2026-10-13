// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/loop_range.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 8e48fbd10b8df1b86579d8eb3cd0795a9b5b996043b93c823e09d773b180ca35
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: loop_range.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 394, 395, 399, 401, 1455, 1458, 1460, 1471, 1478, 1483, 1504, 1523, 1525, 3147, 3148
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopIdx::index
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":372,"rva":20992496,"va":5389701616}
// 编译实例 RVAs: 0x14051f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26920875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389701616,"raw_hex":"e8abc79a01","size":5,"soft_fail":false,"target_va":5416622560,"va":5389701680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26920927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389701616,"raw_hex":"e8dfc79a01","size":5,"soft_fail":false,"target_va":5416622624,"va":5389701692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1458,"record_offset":492,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":26188,"source_line":3227,"archive":"_pdb/C/modules/01458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopIdx::raw_idx
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":528,"rva":977184,"va":5369686304}
// 编译实例 RVAs: 0xee920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$1::sub
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":1628,"rva":20993168,"va":5389702288}
// 编译实例 RVAs: 0x1405490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$15::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65F5
// 首条位置: {"module":394,"inlinee":26101}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$18::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1460,"record_offset":8836,"rva":21012528,"va":5389721648}
// 编译实例 RVAs: 0x140a030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field3_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4900205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389721648,"raw_hex":"e8933ab5ff","size":5,"soft_fail":false,"target_va":5384821600,"va":5389721800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$2::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":1736,"rva":20993184,"va":5389702304}
// 编译实例 RVAs: 0x14054a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$3::next_back
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":260,"rva":20992448,"va":5389701568}
// 编译实例 RVAs: 0x14051c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopRange::len
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 98, 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13636
// 首条位置: {"module":1458,"inlinee":79414}
// 编译实例 RVAs: 0x1405260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopRange::is_empty
// 记录形式: compiled_procedure; instances=1
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":976,"rva":20992640,"va":5389701760}
// 编译实例 RVAs: 0x1405280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::loop_range::LoopRange::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1458,"record_offset":1100,"parent_record_offset":976,"ancestor_record_offset":976,"inlinee":79414,"source_line":98,"archive":"_pdb/C/modules/01458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopRange::start
// 记录形式: compiled_procedure; instances=1
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":864,"rva":20992624,"va":5389701744}
// 编译实例 RVAs: 0x1405270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopRange::end
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":640,"rva":20992592,"va":5389701712}
// 编译实例 RVAs: 0x1405250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopRange::new_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":1284,"rva":20992848,"va":5389701968}
// 编译实例 RVAs: 0x1405350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26920158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389701968,"raw_hex":"e8dec49a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5389702045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1458,"record_offset":1408,"parent_record_offset":1284,"ancestor_record_offset":1284,"inlinee":79414,"source_line":98,"archive":"_pdb/C/modules/01458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopRange::new_line
// 记录形式: compiled_procedure; instances=1
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":1128,"rva":20992656,"va":5389701776}
// 编译实例 RVAs: 0x1405290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26920350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389701776,"raw_hex":"e89ec59a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5389701853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1458,"record_offset":1252,"parent_record_offset":1128,"ancestor_record_offset":1128,"inlinee":79414,"source_line":98,"archive":"_pdb/C/modules/01458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopRange::map_slice
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x660A
// 首条位置: {"module":395,"inlinee":26122}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopRange::subslice
// 记录形式: compiled_procedure; instances=1
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1458,"record_offset":1440,"rva":20993040,"va":5389702160}
// 编译实例 RVAs: 0x1405410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26919955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389702160,"raw_hex":"e813c49a01","size":5,"soft_fail":false,"target_va":5416622208,"va":5389702248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1458,"record_offset":1596,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01458.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1458,"record_offset":1568,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":79414,"source_line":98,"archive":"_pdb/C/modules/01458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::LoopSlice::len
// 记录形式: inline_instance, inlinee_declaration; instances=21
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6646
// 首条位置: {"module":399,"inlinee":26182}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$6::next
// 记录形式: inline_instance, inlinee_declaration; instances=52
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135E5
// 首条位置: {"module":1455,"inlinee":79333}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1455,"record_offset":56076,"parent_record_offset":56048,"ancestor_record_offset":56048,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$6::next::closure$0
//     owner=private_source_ownership_candidate; count=40
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1455,"record_offset":56108,"parent_record_offset":56076,"ancestor_record_offset":56048,"inlinee":79335,"source_line":213,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$6::next::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=53
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135E7
// 首条位置: {"module":1455,"inlinee":79335}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$7::next_back
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1372C
// 首条位置: {"module":1504,"inlinee":79660}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1504,"record_offset":12544,"parent_record_offset":12516,"ancestor_record_offset":12516,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$7::next_back::closure$0
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1504,"record_offset":12576,"parent_record_offset":12544,"ancestor_record_offset":12516,"inlinee":79658,"source_line":219,"archive":"_pdb/C/modules/01504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$7::next_back::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1372A
// 首条位置: {"module":1483,"inlinee":79658}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::loop_range::impl$8::index
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x664B
// 首条位置: {"module":399,"inlinee":26187}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":399,"record_offset":2160,"parent_record_offset":2124,"ancestor_record_offset":2124,"inlinee":26188,"source_line":3227,"archive":"_pdb/C/modules/00399.jsonl.gz"}
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
// 完整结构及原始类型记录: loop_range.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::loop_range::LoopIdx
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::loop_range::LoopRange
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>", "constructor": "core::iter::adapters::map::Map", "arguments": ["utils::loop_range::LoopRange", "system_wall_constructor::construct_lintels::construct_lintels::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>", "core::slice::iter::Iter<f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >", "system_wall_constructor::construct_lintels::construct_lintels::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::loop_range::LoopSlice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 29
// [PDB 泛型关系，非字段关系] {"type": "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "constructor": "utils::loop_range::LoopSlice", "arguments": ["glam::f32::vec3::Vec3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["f32", "core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::loop_range::LoopSlice<f32>", "constructor": "utils::loop_range::LoopSlice", "arguments": ["f32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["utils::loop_range::LoopSlice<f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > >", "constructor": "core::iter::adapters::skip::Skip", "arguments": ["core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > >", "constructor": "core::iter::adapters::take::Take", "arguments": ["core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["f32", "core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >", "constructor": "utils::loop_range::LoopSlice", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >", "constructor": "core::iter::adapters::skip::Skip", "arguments": ["utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > >", "constructor": "core::iter::adapters::take::Take", "arguments": ["core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>", "core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5>", "constructor": "core::iter::adapters::map::Map", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["f32", "core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >", "constructor": "core::iter::adapters::take::Take", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >", "constructor": "core::iter::adapters::skip::Skip", "arguments": ["core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::copied::Copied<core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >", "glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec2::Vec2", "core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::iter::adapters::skip::Skip<core::iter::adapters::take::Take<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >", "constructor": "core::iter::adapters::skip::Skip", "arguments": ["core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >", "constructor": "core::iter::adapters::take::Take", "arguments": ["core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_left", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_left_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::impl$6::extend_right::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::CurveSlice::extend_right_until", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::impl$6::iter_coords::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::ops::range::Range<isize>", "utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >", "core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::impl$6::measure_length::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >", "utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >", "constructor": "itertools::tuple_impl::TupleWindows", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::impl$6::measure_length::closure_env$1", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >", "utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "constructor": "core::iter::traits::accum::impl$128::sum::closure_env$0", "arguments": ["core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<f32,itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>,f32,core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["f32", "itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >", "utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "f32", "core::iter::traits::accum::impl$128::sum::closure_env$0<core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,utils::curve::curve_slice::impl$6::measure_length::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::impl$12::sample_at_coord<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>", "constructor": "utils::curve::impl$12::sample_at_coord", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >", "constructor": "core::iter::adapters::skip::Skip", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::ops::range::RangeFrom<usize>", "core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "constructor": "take_until::take_until::TakeUntil", "arguments": ["core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >", "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >", "constructor": "itertools::tuple_impl::impl$7::next", "arguments": ["core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>", "constructor": "utils::curve::curve_slice::impl$6::extend_left::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "constructor": "take_until::take_until::TakeUntil", "arguments": ["core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >", "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >", "constructor": "core::iter::adapters::rev::Rev", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >", "constructor": "core::iter::adapters::skip::Skip", "arguments": ["core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "constructor": "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >", "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >", "constructor": "itertools::tuple_impl::impl$7::next", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >", "constructor": "itertools::tuple_impl::impl$7::next", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >", "tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>", "constructor": "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >", "constructor": "take_until::take_until::TakeUntil", "arguments": ["core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >", "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >", "core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>", "constructor": "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >", "utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >", "constructor": "itertools::tuple_impl::impl$7::next", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >", "tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >", "constructor": "core::iter::adapters::rev::impl$1::nth", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::take::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >", "constructor": "core::iter::adapters::take::impl$1::nth", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,glam::f32::vec3::Vec3>", "constructor": "core::iter::adapters::copied::impl$1::nth", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "glam::f32::vec3::Vec3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::nth<utils::loop_range::LoopSlice<f32>,f32>", "constructor": "core::iter::adapters::copied::impl$1::nth", "arguments": ["utils::loop_range::LoopSlice<f32>", "f32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::nth<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >", "constructor": "core::iter::traits::iterator::Iterator::nth", "arguments": ["utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >", "ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >", "constructor": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf", "arguments": ["tuple$<usize,glam::f32::vec3::Vec3>", "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::impl$2::next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >", "constructor": "core::iter::traits::iterator::impl$2::next", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> >,core::iter::adapters::map::Map<core::ops::range::Range<isize>,utils::curve::curve_slice::impl$6::iter_coords::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::sources::once::Once<utils::curve::CurveInternalCoord<utils::curve::CurveCoordSemanticThresholded3D> > >,utils::curve::curve_slice::impl$6::measure_length::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>", "constructor": "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>", "utils::curve::CurveCoordSemanticThresholded3D", "glam::f32::vec3::Vec3", "glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)", "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >", "constructor": "take_until::take_until::impl$2::next", "arguments": ["core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >", "utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >", "constructor": "core::iter::traits::iterator::Iterator::nth", "arguments": ["utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >", "ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >", "constructor": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf", "arguments": ["tuple$<usize,glam::f32::vec2::Vec2>", "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >", "ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > >", "constructor": "itertools::tuple_impl::impl$31::collect_from_iter_no_buf", "arguments": ["tuple$<usize,glam::f32::vec2::Vec2>", "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: loop_range.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/utils/src/loop_range.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/utils/src/loop_range.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/utils/src/loop_range.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: loop_range.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopIdx::index
// 信息 ID: 1a18ae9ba698ce7f743966cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 90f214cb4e5003256a054e19: glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 引用方: 原程序直接转移 59 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7eba98 -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ed25d -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ed301 -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ed3dd -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecac4 -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecdff -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_wood_arch_profile @ 0x80c6fd -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0x80d1d0 -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811f1f -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> > @ 0x13f5049 -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> > @ 0x13f51e5 -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > > @ 0x13f56be -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > > @ 0x13f57a9 -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> > @ 0x13f597b -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> > @ 0x13f5a8e -> 0x14051f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > > @ 0x13f848e -> 0x14051f0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopIdx::raw_idx
// 信息 ID: 989c3b7f422dfb76f4f78ba7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8c79 -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::SdlWindowBackend::setup_window @ 0x3f8bfb -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::monitor_match_from_sdl @ 0x3f9827 -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ff610 -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ff648 -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0x80c95a -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0x80c977 -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] tokio::runtime::scheduler::multi_thread::queue::impl$5::drop<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle,alloc::alloc::Global> > @ 0x10fd3cb -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] tokio::runtime::scheduler::multi_thread::queue::impl$5::drop<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle,alloc::alloc::Global> > @ 0x10fd43d -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] tokio::runtime::scheduler::multi_thread::queue::impl$5::drop<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle,alloc::alloc::Global> > @ 0x1376fcb -> 0xee920 (direct_call); 同地址别名未裁定。
// [静态直接引用] tokio::runtime::scheduler::multi_thread::queue::impl$5::drop<alloc::sync::Arc<tokio::runtime::scheduler::multi_thread::handle::Handle,alloc::alloc::Global> > @ 0x137703d -> 0xee920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopRange::end
// 信息 ID: afc006515a46e63a6094e6d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecde9 -> 0x1405250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ff628 -> 0x1405250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0x80c968 -> 0x1405250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopRange::is_empty
// 信息 ID: 59b1ea023602d5ed06667450
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl @ 0x14094f1 -> 0x1405280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopRange::len
// 信息 ID: cbe8072caf3832215ce5dda2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 18 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x7faa5f -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x7faaba -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x7faae4 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x7fabd5 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x7fabef -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x7fadbf -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x7fb018 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x7fb7c9 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x7fb7f1 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x7fb802 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate @ 0x7fb822 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range @ 0x7fbc8b -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ff694 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count @ 0xa05d44 -> 0x1405260 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> @ 0x142a79a -> 0x1405260 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::curve::curve_slice::CurveSlice::extend_left<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> @ 0x142a7c9 -> 0x1405260 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::loop_range::LoopRange::is_empty; module=1458, inline_record=1100; 不是独立调用边。
// [PDB 内联宿主] utils::loop_range::LoopRange::new_line; module=1458, inline_record=1252; 不是独立调用边。
// [PDB 内联宿主] utils::loop_range::LoopRange::new_loop; module=1458, inline_record=1408; 不是独立调用边。
// [PDB 内联宿主] utils::loop_range::LoopRange::subslice; module=1458, inline_record=1568; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopRange::map_slice
// 信息 ID: 5604219c94e0a4fe6f1e56e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=395, inline_record=11440; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=399, inline_record=14004; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=399, inline_record=14032; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=401, inline_record=11048; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1460, inline_record=840; 不是独立调用边。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>; module=1471, inline_record=2288; 不是独立调用边。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>; module=1471, inline_record=2316; 不是独立调用边。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2>; module=1471, inline_record=2904; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopRange::new_line
// 信息 ID: 14c3f7ccb386ec338e630928
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x7fc569 -> 0x1405290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopRange::new_loop
// 信息 ID: 91b2cd68873e634b7c9728bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x7fc586 -> 0x1405350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopRange::start
// 信息 ID: d2fdf43e730e3e47497e3ee1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7eb6c9 -> 0x1405270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ed255 -> 0x1405270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ed333 -> 0x1405270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecaae -> 0x1405270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7ff5f0 -> 0x1405270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0x80c94b -> 0x1405270 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811f4f -> 0x1405270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopRange::subslice
// 信息 ID: 699e253031e610dd4bf7ebb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7f1b44 -> 0x1405410 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl @ 0x14094e0 -> 0x1405410 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8> @ 0x141a3c3 -> 0x1405410 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2> @ 0x141a685 -> 0x1405410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::LoopSlice::len
// 信息 ID: 45243ed6d7fadbcb6897825a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 17 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=516; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=924; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=1020; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=1732; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=1760; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=2536; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=3952; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6084; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$15::clone
// 信息 ID: 0d8185438671c8cba37ba591
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=394, inline_record=19856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$18::fmt
// 信息 ID: b98d9a0d0c81fa44854cf3f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$1::sub
// 信息 ID: 8f9a56d319ea8d7b9cd2bf45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7eb6e9 -> 0x1405490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ed343 -> 0x1405490 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811f68 -> 0x1405490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$2::next
// 信息 ID: fc8344176269b020df13a2af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 90f214cb4e5003256a054e19: glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 引用方: 原程序直接转移 48 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7eb64f -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ed2de -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ed3be -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_wood_arch_profile @ 0x80c6df -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0x80d1ae -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811ef5 -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> > @ 0x13f502d -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> > @ 0x13f51c7 -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > > @ 0x13f56a6 -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > > @ 0x13f578f -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> > @ 0x13f5963 -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> > @ 0x13f5a74 -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > > @ 0x13f8476 -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > > @ 0x13f8550 -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> > @ 0x13fa5ae -> 0x14054a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> > @ 0x13fa68c -> 0x14054a0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$3::next_back
// 信息 ID: 4a27704b64eb38f617c3fcef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > > @ 0x1473630 -> 0x14051c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > > @ 0x1473b22 -> 0x14051c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > | core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > | core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > @ 0x1473e76 -> 0x14051c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > | core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > | core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > @ 0x1473ea5 -> 0x14051c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > > @ 0x14812ad -> 0x14051c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > > @ 0x1481366 -> 0x14051c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > > @ 0x24e137f -> 0x14051c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > > > > > @ 0x24e1419 -> 0x14051c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$6::next
// 信息 ID: 3180148eb7dc135cc95c2d40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 90f214cb4e5003256a054e19: glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 引用方: 原程序直接转移 0 条；内联宿主 40 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> >; module=1455, inline_record=56048; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> >; module=1455, inline_record=57020; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > >; module=1455, inline_record=61056; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > >; module=1455, inline_record=61776; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > >; module=1455, inline_record=99300; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > >; module=1455, inline_record=100180; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> >; module=1455, inline_record=137544; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> >; module=1455, inline_record=138320; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$6::next::closure$0
// 信息 ID: 0965abd49db15869b0c944e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 90f214cb4e5003256a054e19: glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::loop_range::impl$6::next", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 40 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> >; module=1455, inline_record=56108; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$0> >; module=1455, inline_record=57080; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > >; module=1455, inline_record=61116; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32> > > > >; module=1455, inline_record=61836; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > >; module=1455, inline_record=99360; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > >; module=1455, inline_record=100240; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> >; module=1455, inline_record=137604; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> >; module=1455, inline_record=138380; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$7::next_back
// 信息 ID: 37e6ae92dfff7cfc3b36de47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; module=1504, inline_record=12516; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1504, inline_record=17940; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >; module=1504, inline_record=20480; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >; module=1504, inline_record=20660; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=5928; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=7032; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=7696; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >; module=1525, inline_record=12896; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$7::next_back::closure$0
// 信息 ID: 4a113cc8e8e1e84baa65b95b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::loop_range::impl$7::next_back", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl$6::extend_left::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)> > >,tuple$<tuple$<usize,glam::f32::vec2::Vec2>,tuple$<usize,glam::f32::vec2::Vec2> > >; module=1504, inline_record=12576; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1504, inline_record=18000; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >; module=1504, inline_record=20540; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >; module=1504, inline_record=20720; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=5980; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=7084; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1525, inline_record=7744; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::rev::impl$1::nth<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> >; module=1525, inline_record=12956; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::loop_range::impl$8::index
// 信息 ID: 1a53fffbabc186ba78f3c0dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=2124; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=399, inline_record=2208; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=399, inline_record=6384; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=401, inline_record=11432; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=401, inline_record=11532; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1460, inline_record=6396; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1460, inline_record=6488; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1460, inline_record=6920; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
