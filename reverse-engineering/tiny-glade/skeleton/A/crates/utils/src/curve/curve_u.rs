// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/curve/curve_u.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0d7391ccb5ad70b33eaa5aa2df75412987731bc71bc7241055d243e446d88b95
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: curve_u.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 55, 100, 265, 361, 372, 373, 374, 377, 385, 389, 391, 392, 409, 423, 427, 430, 435, 444, 452, 454, 461, 463, 472, 474, 476, 486, 492, 493, 496, 740, 741, 743, 747, 753, 754, 1206, 1216, 1218, 1219, 1220, 1222, 1224, 1225, 1226, 1227, 1229, 1230, 1231, 1232, 1233, 1241, 1243, 1244, 1248, 1249, 1250, 1251, 1252, 1253, 1257, 1261, 1263, 1264, 1272, 1274, 1286, 1288, 1294, 1302, 1308, 1309, 1321, 1327, 1328, 1339, 1350, 1355, 1357, 1367, 1389, 1399, 1403, 1426, 1432, 1440, 1447, 1893, 1903, 1904, 1924, 2272, 2698, 2715, 2726, 2746, 2774, 2815, 2817, 2822, 2825, 3086, 3192, 3215, 3217, 3456, 3508, 3513, 3514, 3524, 3842, 3843, 4113, 4114
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::impl$12::eq
// 记录形式: inline_instance, inlinee_declaration; instances=42
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FDB
// 首条位置: {"module":423,"inlinee":24539}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::impl$13::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FE5
// 首条位置: {"module":423,"inlinee":24549}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$50::partial_cmp
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":24540,"parent_record_offset":24516,"ancestor_record_offset":24516,"inlinee":7043,"source_line":1506,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::impl$8::fmt<utils::curve::CurveCoordSemantic2D>
// 记录形式: compiled_procedure; instances=3
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1229,"record_offset":29432,"rva":13334512,"va":5382043632}
// 编译实例 RVAs: 0xcb77f0, 0xe8ea00, 0x14de4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4459537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382043632,"raw_hex":"e8eff3bbff","size":5,"soft_fail":false,"target_va":5377584208,"va":5382043740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::impl$8::fmt<utils::curve::CurveCoordSemantic3D>
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1327,"record_offset":75996,"rva":15264128,"va":5383973248}
// 编译实例 RVAs: 0xe8e980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6389153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383973248,"raw_hex":"e85f829eff","size":5,"soft_fail":false,"target_va":5377584208,"va":5383973356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EB6
// 首条位置: {"module":55,"inlinee":7862}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_f32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":100,"record_offset":64476,"parent_record_offset":64452,"ancestor_record_offset":64452,"inlinee":7834,"source_line":133,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::abs_private
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":100,"record_offset":64572,"parent_record_offset":64548,"ancestor_record_offset":64452,"inlinee":7839,"source_line":510,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_finite
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f64.rs
//     evidence={"module":100,"record_offset":64548,"parent_record_offset":64524,"ancestor_record_offset":64452,"inlinee":7838,"source_line":561,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":100,"record_offset":64608,"parent_record_offset":64500,"ancestor_record_offset":64452,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::Number::from_f64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/number.rs
//     evidence={"module":100,"record_offset":64524,"parent_record_offset":64500,"ancestor_record_offset":64452,"inlinee":7837,"source_line":264,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_f64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":100,"record_offset":64500,"parent_record_offset":64476,"ancestor_record_offset":64452,"inlinee":7835,"source_line":138,"archive":"_pdb/A/modules/00100.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveU::max
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FD6
// 首条位置: {"module":423,"inlinee":24534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":423,"record_offset":14412,"parent_record_offset":14388,"ancestor_record_offset":14388,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveU::min
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6707
// 首条位置: {"module":461,"inlinee":26375}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":461,"record_offset":61932,"parent_record_offset":61908,"ancestor_record_offset":61908,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68B0
// 首条位置: {"module":472,"inlinee":26800}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::impl$2::visit_f64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68CF
// 首条位置: {"module":1389,"inlinee":26831}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1389,"record_offset":3728,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01389.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1389,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":21345,"source_line":189,"archive":"_pdb/A/modules/01389.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1389,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":3604,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01389.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1389,"record_offset":3696,"parent_record_offset":3632,"ancestor_record_offset":3604,"inlinee":21346,"source_line":535,"archive":"_pdb/A/modules/01389.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1389,"record_offset":3668,"parent_record_offset":3632,"ancestor_record_offset":3604,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::impl$3::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68CE
// 首条位置: {"module":1440,"inlinee":26830}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::missing_field::impl$0::deserialize_f64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":396520,"parent_record_offset":396496,"ancestor_record_offset":396496,"inlinee":26801,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":396544,"parent_record_offset":396520,"ancestor_record_offset":396496,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveU::new
// 记录形式: inline_instance, inlinee_declaration; instances=261
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x393E
// 首条位置: {"module":265,"inlinee":14654}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=168
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":372,"record_offset":21508,"parent_record_offset":21356,"ancestor_record_offset":21356,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=164
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":372,"record_offset":21536,"parent_record_offset":21356,"ancestor_record_offset":21356,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=177
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":372,"record_offset":21396,"parent_record_offset":21356,"ancestor_record_offset":21356,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=172
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":21444,"parent_record_offset":21420,"ancestor_record_offset":21356,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=172
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":21468,"parent_record_offset":21444,"ancestor_record_offset":21356,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=177
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":372,"record_offset":21420,"parent_record_offset":21396,"ancestor_record_offset":21356,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveU::new<utils::curve::CurveCoordSemantic2D>
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":472,"record_offset":3312,"rva":6675504,"va":5375384624}
// 编译实例 RVAs: 0x65dc30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27594383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375384624,"raw_hex":"e88f0ea501","size":5,"soft_fail":false,"target_va":5402979152,"va":5375384764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27594192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375384624,"raw_hex":"e8d00da501","size":5,"soft_fail":false,"target_va":5402978944,"va":5375384747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":472,"record_offset":3584,"parent_record_offset":3312,"ancestor_record_offset":3312,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":472,"record_offset":3612,"parent_record_offset":3312,"ancestor_record_offset":3312,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":472,"record_offset":3476,"parent_record_offset":3312,"ancestor_record_offset":3312,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":472,"record_offset":3524,"parent_record_offset":3500,"ancestor_record_offset":3312,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":472,"record_offset":3548,"parent_record_offset":3524,"ancestor_record_offset":3312,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":472,"record_offset":3500,"parent_record_offset":3476,"ancestor_record_offset":3312,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveU::new<utils::curve::CurveCoordSemantic3D>
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1233,"record_offset":260,"rva":13350896,"va":5382060016}
// 编译实例 RVAs: 0xcbb7f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20918991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382060016,"raw_hex":"e8cf323f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382060156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20918800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382060016,"raw_hex":"e810323f01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382060139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1233,"record_offset":532,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01233.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1233,"record_offset":560,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01233.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1233,"record_offset":424,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01233.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1233,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01233.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1233,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01233.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1233,"record_offset":448,"parent_record_offset":424,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01233.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveU::u
// 记录形式: inline_instance, inlinee_declaration; instances=192
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x536A
// 首条位置: {"module":361,"inlinee":21354}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveU::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=38
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5515
// 首条位置: {"module":385,"inlinee":21781}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":385,"record_offset":5876,"parent_record_offset":5840,"ancestor_record_offset":5840,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1261,"record_offset":2152,"parent_record_offset":2012,"ancestor_record_offset":2012,"inlinee":36057,"source_line":263,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":385,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":5840,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":385,"record_offset":5948,"parent_record_offset":5924,"ancestor_record_offset":5840,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":385,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":5840,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveU::clamp<utils::curve::CurveCoordSemanticThresholded3D>
// 记录形式: compiled_procedure; instances=1
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":472,"record_offset":3644,"rva":6675664,"va":5375384784}
// 编译实例 RVAs: 0x65dcd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27594072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375384784,"raw_hex":"e8580da501","size":5,"soft_fail":false,"target_va":5402979152,"va":5375385075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27593976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375384784,"raw_hex":"e8f80ca501","size":5,"soft_fail":false,"target_va":5402978944,"va":5375384963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":472,"record_offset":3844,"parent_record_offset":3644,"ancestor_record_offset":3644,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":472,"record_offset":3924,"parent_record_offset":3644,"ancestor_record_offset":3644,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":472,"record_offset":3820,"parent_record_offset":3644,"ancestor_record_offset":3644,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":472,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3644,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":472,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3644,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":472,"record_offset":4068,"parent_record_offset":3924,"ancestor_record_offset":3644,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":472,"record_offset":3892,"parent_record_offset":3844,"ancestor_record_offset":3644,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":472,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3644,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":472,"record_offset":3960,"parent_record_offset":3924,"ancestor_record_offset":3644,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveUBoundless::new
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5361
// 首条位置: {"module":361,"inlinee":21345}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":361,"record_offset":15560,"parent_record_offset":15492,"ancestor_record_offset":15492,"inlinee":21346,"source_line":535,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":361,"record_offset":15532,"parent_record_offset":15492,"ancestor_record_offset":15492,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveUBoundless::new<utils::curve::CurveCoordSemanticThresholded3D>
// 记录形式: compiled_procedure; instances=1
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":472,"record_offset":4132,"rva":6675968,"va":5375385088}
// 编译实例 RVAs: 0x65de00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27594011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375385088,"raw_hex":"e81b0da501","size":5,"soft_fail":false,"target_va":5402979152,"va":5375385136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":472,"record_offset":4336,"parent_record_offset":4132,"ancestor_record_offset":4132,"inlinee":21346,"source_line":535,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":472,"record_offset":4312,"parent_record_offset":4132,"ancestor_record_offset":4132,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveUBoundless::clamp
// 记录形式: inline_instance, inlinee_declaration; instances=63
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5363
// 首条位置: {"module":361,"inlinee":21347}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":361,"record_offset":15616,"parent_record_offset":15592,"ancestor_record_offset":15592,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00361.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::curve::curve_u::CurveUBoundless::is_within_01_range
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5639
// 首条位置: {"module":389,"inlinee":22073}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: curve_u.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::curve::curve_u::CurveU
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 61
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "constructor": "utils::curve::curve_u::CurveU", "arguments": ["utils::curve::CurveCoordSemantic2D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global> > >", "constructor": "serde::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveU::new<utils::curve::CurveCoordSemantic2D>", "constructor": "utils::curve::curve_u::CurveU::new", "arguments": ["utils::curve::CurveCoordSemantic2D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveU::clamp<utils::curve::CurveCoordSemanticThresholded3D>", "constructor": "utils::curve::curve_u::CurveU::clamp", "arguments": ["utils::curve::CurveCoordSemanticThresholded3D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>", "constructor": "utils::curve::curve_u::CurveU", "arguments": ["utils::curve::CurveCoordSemanticThresholded3D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >", "constructor": "ref$", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >", "constructor": "tuple$", "arguments": ["ref$<glam::f32::vec3::Vec3>", "ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >", "constructor": "tuple$", "arguments": ["tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >", "ref$<f32>", "ref$<f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> > >", "constructor": "tuple$", "arguments": ["tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$4::call_once<tuple$<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> > >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>", "constructor": "core::ops::function::impls::impl$4::call_once", "arguments": ["tuple$<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> > >", "country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "constructor": "tuple$", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>,core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["alloc::vec::into_iter::IntoIter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>", "core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>,core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>,core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >", "constructor": "core::slice::iter::Iter", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >", "core::slice::iter::Iter<f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >", "core::slice::iter::Iter<f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >", "country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>", "alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >", "country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>", "constructor": "tuple$", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "glam::f32::vec2::Vec2", "glam::f32::vec2::Vec2", "f32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>", "core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> >", "constructor": "ref$", "arguments": ["tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >", "constructor": "core::iter::sources::once::Once", "arguments": ["ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >,core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >", "core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >", "constructor": "ref_mut$", "arguments": ["core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >,core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > > >,ref_mut$<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >,core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > > >", "ref_mut$<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >,core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > > >,ref_mut$<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> >", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > >,core::iter::sources::once::Once<ref$<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > > >,ref_mut$<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>", "constructor": "utils::curve::curve_u::CurveU", "arguments": ["utils::curve::CurveCoordSemantic3D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "constructor": "core::slice::iter::Iter", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$12> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$12>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >", "constructor": "utils::loop_range::LoopSlice", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >", "constructor": "core::iter::adapters::skip::Skip", "arguments": ["utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > >", "constructor": "core::iter::adapters::take::Take", "arguments": ["core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>", "core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>", "alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >", "alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >", "alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >", "alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::break_patterns<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "constructor": "core::slice::sort::break_patterns", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14>", "constructor": "core::slice::impl$0::sort_unstable_by::closure_env$0", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$14"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::partial_insertion_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >", "constructor": "core::slice::sort::partial_insertion_sort", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15>", "constructor": "core::slice::impl$0::sort_unstable_by::closure_env$0", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "system_wall_constructor::utils::half_timber::half_timbering::closure_env$15"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::partial_insertion_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >", "constructor": "core::slice::sort::partial_insertion_sort", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_right<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >", "constructor": "core::slice::sort::insertion_sort_shift_right", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_right<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >", "constructor": "core::slice::sort::insertion_sort_shift_right", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >", "constructor": "core::slice::sort::recurse", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >", "constructor": "core::slice::sort::recurse", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >", "constructor": "ref_mut$", "arguments": ["core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::heapsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,ref_mut$<core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> > >", "constructor": "core::slice::sort::heapsort", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "ref_mut$<core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >", "constructor": "ref_mut$", "arguments": ["core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::heapsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,ref_mut$<core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> > >", "constructor": "core::slice::sort::heapsort", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "ref_mut$<core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "constructor": "utils::curve::Curve::get_coord_at_u", "arguments": ["glam::f32::vec2::Vec2", "utils::curve::CurveCoordSemantic2D", "utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveU::new<utils::curve::CurveCoordSemantic3D>", "constructor": "utils::curve::curve_u::CurveU::new", "arguments": ["utils::curve::CurveCoordSemantic3D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "constructor": "ref$", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >", "constructor": "core::iter::sources::once::Once", "arguments": ["ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >", "core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >", "constructor": "ref_mut$", "arguments": ["core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >", "ref_mut$<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> >", "constructor": "core::iter::adapters::cycle::Cycle", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >", "ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D>", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::sources::once::Once<utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::Curve::get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "constructor": "utils::curve::Curve::get_coord_at_u", "arguments": ["glam::f32::vec3::Vec3", "utils::curve::CurveCoordSemantic2D", "utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$10::clone<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$10::clone", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >", "country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>", "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >", "constructor": "core::slice::iter::Iter", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >", "country_core::systems::wall::wall_outline::wall_outline::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["glam::f32::vec3::Vec3", "alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["glam::f32::vec3::Vec3", "core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >", "country_core::systems::wall::wall_outline::wall_outline::closure_env$1", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >,country_core::systems::wall::wall_outline::wall_outline::closure_env$1> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::Curve::get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >", "constructor": "utils::curve::Curve::get_coord_at_u", "arguments": ["glam::f32::vec3::Vec3", "utils::curve::CurveCoordSemanticThresholded3D", "utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >", "constructor": "serde_json::value::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >", "constructor": "serde_json::value::de::impl$2::deserialize_seq", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>", "constructor": "tuple$", "arguments": ["glam::f32::vec3::Vec3", "f32", "ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >", "glam::f32::vec3::Vec3", "f32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >", "constructor": "core::iter::sources::once::Once", "arguments": ["tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >,core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >", "core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0> >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::next<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >,core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0> > >", "constructor": "core::iter::adapters::chain::impl$1::next", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >,core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> > >", "ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>,tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >", "constructor": "tuple$", "arguments": ["tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>", "tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>,tuple$<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>,tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> > >", "constructor": "itertools::tuple_impl::impl$6::next", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>", "tuple$<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>,tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>,tuple$<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>,tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> > >", "constructor": "itertools::tuple_impl::tuple_windows", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,core::slice::iter::Iter<f32> >,core::slice::iter::Iter<f32> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$1>,alloc::vec::into_iter::IntoIter<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> >,alloc::alloc::Global> >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>", "tuple$<tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32>,tuple$<glam::f32::vec3::Vec3,f32,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >,glam::f32::vec3::Vec3,f32> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$9::serialize_field<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >", "constructor": "serde_json::value::ser::impl$9::serialize_field", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde::de::impls::impl$19::deserialize::VecVisitor<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$19::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>", "alloc::vec::impl$19::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$19::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>", "core::ops::range::Range<i32>", "system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::vec::impl$19::extend_trusted::closure_env$0<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2,f32>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<i32>,system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof::closure_env$2<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::impl$1::next<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > >,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> >", "constructor": "core::iter::adapters::copied::impl$1::next", "arguments": ["core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > > >", "utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::curve::curve_u::CurveUBoundless
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveUBoundless::new<utils::curve::CurveCoordSemanticThresholded3D>", "constructor": "utils::curve::curve_u::CurveUBoundless::new", "arguments": ["utils::curve::CurveCoordSemanticThresholded3D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::curve::curve_u::CurveUVisitor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic3D>", "constructor": "utils::curve::curve_u::CurveUVisitor", "arguments": ["utils::curve::CurveCoordSemantic3D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impl$1::fmt<utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic3D> >", "constructor": "serde::de::impl$1::fmt", "arguments": ["utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic2D>", "constructor": "utils::curve::curve_u::CurveUVisitor", "arguments": ["utils::curve::CurveCoordSemantic2D"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic2D> >", "constructor": "core::ptr::drop_in_place", "arguments": ["utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_f64<utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic3D> >", "constructor": "serde_json::value::de::impl$2::deserialize_f64", "arguments": ["utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic3D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$2::deserialize_f64<utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic2D> >", "constructor": "serde_json::value::de::impl$2::deserialize_f64", "arguments": ["utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic2D>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: curve_u.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: curve_u.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 82; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::firstperson_steps::FirstPersonViewAudioState>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::firstperson_steps::FirstPersonViewAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::firstperson_steps::FirstPersonViewAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>,bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::event::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>", "bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveU::clamp<utils::curve::CurveCoordSemanticThresholded3D>
// 信息 ID: c092042944dfb8c21f88f96b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::curve::curve_u::CurveU::clamp -> ["utils::curve::CurveCoordSemanticThresholded3D"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2> @ 0x66c6cb -> 0x65dcd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2> @ 0x66c6e2 -> 0x65dcd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$2> @ 0x66d150 -> 0x65dcd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_tris::closure_env$2> @ 0x66d164 -> 0x65dcd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveU::lerp
// 信息 ID: 8130458907fe7900902d31fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors; module=385, inline_record=5840; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=15016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=15728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=15876; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=17144; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::closest_segment_index; module=454, inline_record=19784; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=61324; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer; module=1222, inline_record=4340; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveU::max
// 信息 ID: 40563ddc08832435341b9216
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=14388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveU::min
// 信息 ID: ed69a42f6c36f57b912d69ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=61908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveU::new
// 信息 ID: 2fdd29feec420a17e5a5d0d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 187 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=21356; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=21568; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=22016; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=22228; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=22688; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=22900; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=23360; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=23572; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveU::new<utils::curve::CurveCoordSemantic2D>
// 信息 ID: 647663f9aa5dd96bab76a314
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::curve::curve_u::CurveU::new -> ["utils::curve::CurveCoordSemantic2D"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveU::new<utils::curve::CurveCoordSemantic3D>
// 信息 ID: d705a5433c1f1791d13443de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::curve::curve_u::CurveU::new -> ["utils::curve::CurveCoordSemantic3D"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0xcb9935 -> 0xcbb7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0xcb9951 -> 0xcbb7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::profile::create_stone_arch_profile @ 0xcb997d -> 0xcbb7f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveU::u
// 信息 ID: 37b76ba199dc34c991beeca9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 146 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=15944; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=16068; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=6640; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=6792; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height; module=377, inline_record=7368; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height; module=377, inline_record=7756; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=19828; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=19992; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveUBoundless::clamp
// 信息 ID: d65b7e6f863e10becf31c560
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 36 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=15592; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=24768; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=24828; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=25136; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=34344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=40616; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=12920; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=15608; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveUBoundless::is_within_01_range
// 信息 ID: 3a1592e0bec911ec58f1dcbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=47396; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=38720; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::ivy_grower; module=454, inline_record=17716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveUBoundless::new
// 信息 ID: e727b39774643425b27984cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=15492; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=38624; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=40392; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=40520; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::terrain_edit::TerrainEdit::update_terrain_curve; module=493, inline_record=10292; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height; module=1216, inline_record=110388; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1229, inline_record=19128; 不是独立调用边。
// [PDB 内联宿主] utils::wall_space::WallSpaceCoord::to_world_space; module=1261, inline_record=3148; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::CurveUBoundless::new<utils::curve::CurveCoordSemanticThresholded3D>
// 信息 ID: 4d1d53b610fa50df22529e17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::curve::curve_u::CurveUBoundless::new -> ["utils::curve::CurveCoordSemanticThresholded3D"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers @ 0xd22848 -> 0x65de00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers @ 0xd22868 -> 0x65de00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::impl$0::serialize
// 信息 ID: d1230fcd8a3347dd44af75c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global> > >; module=100, inline_record=64452; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$9::serialize_field<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >; module=1432, inline_record=7148; 不是独立调用边。
// [PDB 内联宿主] serde::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global> > >; module=1432, inline_record=82832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::impl$12::eq
// 信息 ID: b0b47e8d649d49280add077d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 30 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=14828; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=15260; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=15540; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=16124; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=18780; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish; module=435, inline_record=31992; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2>; module=476, inline_record=37276; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback<country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$0,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$1,country_core::resources::walls::wall_trigger_volume::impl$2::get_collision_geo::closure_env$2>; module=476, inline_record=37328; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::impl$13::partial_cmp
// 信息 ID: 56f09ff6931f23e6fd4cdf19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=24516; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split; module=427, inline_record=51180; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split; module=427, inline_record=51264; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=70272; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::deferred_wall_history::PendingWallChanged::try_new; module=492, inline_record=45544; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=10232; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_pos_at_u; module=740, inline_record=10404; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::_get_normal_at_u; module=740, inline_record=15620; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::impl$2::expecting
// 信息 ID: 474667d9f7c211c856370815
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic3D> >; module=472, inline_record=436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::impl$2::visit_f64
// 信息 ID: ef046850b97e539405576ebb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$2::deserialize_f64<utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic3D> >; module=1389, inline_record=3604; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$2::deserialize_f64<utils::curve::curve_u::CurveUVisitor<utils::curve::CurveCoordSemantic2D> >; module=1389, inline_record=5056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::impl$3::deserialize
// 信息 ID: f3bdf80fd4f7a159c7b7cba7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=396060; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=396396; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=396496; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=396608; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=529956; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=530292; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=530392; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=530504; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::impl$8::fmt<utils::curve::CurveCoordSemantic2D>
// 信息 ID: 83ddd6947bdc141edbdd39c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::curve::curve_u::impl$8::fmt -> ["utils::curve::CurveCoordSemantic2D"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::curve::curve_u::impl$8::fmt<utils::curve::CurveCoordSemantic3D>
// 信息 ID: 08cbdeb0c576ccc4d715bce2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::curve::curve_u::impl$8::fmt -> ["utils::curve::CurveCoordSemantic3D"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
