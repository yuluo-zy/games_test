// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/walls/public_wall_state.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3a4216214751b376d784e697763245b68fcda53c333d654d992fa0e49372bc12
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: public_wall_state.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 361, 362, 367, 369, 371, 374, 377, 383, 391, 392, 394, 396, 397, 401, 411, 417, 434, 452, 453, 454, 461, 486, 492, 496, 1012, 1206, 1216, 1230, 1231, 1248, 1250, 1253, 1257, 1258, 1286, 1302, 1308, 1321, 1357, 1461, 1465, 2745, 2773, 2831, 3441, 3478, 3479, 3509, 3612, 3617, 3842, 3856, 3858, 3859, 3863, 3874, 4377
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
// 记录形式: inline_instance, inlinee_declaration; instances=115
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x537F
// 首条位置: {"module":361,"inlinee":21375}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
// 记录形式: inline_instance, inlinee_declaration; instances=38
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5386
// 首条位置: {"module":361,"inlinee":21382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x56F1
// 首条位置: {"module":391,"inlinee":22257}
// 编译实例 RVAs: 0xc9c6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21046056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381932784,"raw_hex":"e828234101","size":5,"soft_fail":false,"target_va":5402978880,"va":5381932819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":391,"record_offset":25008,"parent_record_offset":24952,"ancestor_record_offset":24952,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":391,"record_offset":24980,"parent_record_offset":24952,"ancestor_record_offset":24952,"inlinee":22258,"source_line":114,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x56F2
// 首条位置: {"module":391,"inlinee":22258}
// 编译实例 RVAs: 0xc9c720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::max_y
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=15
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C1E
// 首条位置: {"module":411,"inlinee":23582}
// 编译实例 RVAs: 0xc9c740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21045996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381932864,"raw_hex":"e8ec224101","size":5,"soft_fail":false,"target_va":5402978944,"va":5381932943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":411,"record_offset":19972,"parent_record_offset":19944,"ancestor_record_offset":19944,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":411,"record_offset":20028,"parent_record_offset":20004,"ancestor_record_offset":19944,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":411,"record_offset":20004,"parent_record_offset":19972,"ancestor_record_offset":19944,"inlinee":23592,"source_line":127,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C28
// 首条位置: {"module":411,"inlinee":23592}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":411,"record_offset":20028,"parent_record_offset":20004,"ancestor_record_offset":20004,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00411.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
// 记录形式: compiled_procedure; instances=1
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1216,"record_offset":108924,"rva":13223840,"va":5381932960}
// 编译实例 RVAs: 0xc9c7a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21045900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381932960,"raw_hex":"e88c224101","size":5,"soft_fail":false,"target_va":5402978944,"va":5381933039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1216,"record_offset":109088,"parent_record_offset":108924,"ancestor_record_offset":108924,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1216,"record_offset":109136,"parent_record_offset":109116,"ancestor_record_offset":108924,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y_gable::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1216,"record_offset":109116,"parent_record_offset":109088,"ancestor_record_offset":108924,"inlinee":59491,"source_line":132,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::impl$0::max_y_gable::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE863
// 首条位置: {"module":1216,"inlinee":59491}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1216,"record_offset":109136,"parent_record_offset":109116,"ancestor_record_offset":109116,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::wall_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1216,"record_offset":109176,"rva":13223936,"va":5381933056}
// 编译实例 RVAs: 0xc9c800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1216,"record_offset":109340,"parent_record_offset":109176,"ancestor_record_offset":109176,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::u_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1216,"record_offset":109368,"rva":13223968,"va":5381933088}
// 编译实例 RVAs: 0xc9c820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1216,"record_offset":109528,"parent_record_offset":109368,"ancestor_record_offset":109368,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A1B
// 首条位置: {"module":401,"inlinee":23067}
// 编译实例 RVAs: 0xc9c840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::is_circle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE86A
// 首条位置: {"module":1216,"record_offset":109712,"rva":13224048,"va":5381933168}
// 编译实例 RVAs: 0xc9c870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1216,"record_offset":109868,"rva":13224096,"va":5381933216}
// 编译实例 RVAs: 0xc9c8a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3974792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933216,"raw_hex":"e87859c3ff","size":5,"soft_fail":false,"target_va":5377958496,"va":5381933283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3977639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933216,"raw_hex":"e8594ec3ff","size":5,"soft_fail":false,"target_va":5377955632,"va":5381933266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
// 记录形式: compiled_procedure; instances=1
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1216,"record_offset":110036,"rva":13224224,"va":5381933344}
// 编译实例 RVAs: 0xc9c920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88710
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: public_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2014596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933344,"raw_hex":"e884bd1e00","size":5,"soft_fail":false,"target_va":5383948048,"va":5381933447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8abc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: public_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2023897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933344,"raw_hex":"e8d9e11e00","size":5,"soft_fail":false,"target_va":5383957440,"va":5381933538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21045414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933344,"raw_hex":"e8a6204101","size":5,"soft_fail":false,"target_va":5402979152,"va":5381933733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t21045730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933344,"raw_hex":"e8e2214101","size":5,"soft_fail":false,"target_va":5402979344,"va":5381933609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21045230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381933344,"raw_hex":"e8ee1f4101","size":5,"soft_fail":false,"target_va":5402978944,"va":5381933709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1216,"record_offset":110240,"parent_record_offset":110036,"ancestor_record_offset":110036,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1216,"record_offset":110332,"parent_record_offset":110036,"ancestor_record_offset":110036,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1216,"record_offset":110360,"parent_record_offset":110036,"ancestor_record_offset":110036,"inlinee":23582,"source_line":126,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1216,"record_offset":110540,"parent_record_offset":110036,"ancestor_record_offset":110036,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1216,"record_offset":110484,"parent_record_offset":110036,"ancestor_record_offset":110036,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1216,"record_offset":110388,"parent_record_offset":110036,"ancestor_record_offset":110036,"inlinee":21345,"source_line":189,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1216,"record_offset":111120,"parent_record_offset":110036,"ancestor_record_offset":110036,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1216,"record_offset":110624,"parent_record_offset":110568,"ancestor_record_offset":110036,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1216,"record_offset":110508,"parent_record_offset":110484,"ancestor_record_offset":110036,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1216,"record_offset":110452,"parent_record_offset":110388,"ancestor_record_offset":110036,"inlinee":21346,"source_line":535,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1216,"record_offset":110424,"parent_record_offset":110388,"ancestor_record_offset":110036,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1216,"record_offset":110296,"parent_record_offset":110272,"ancestor_record_offset":110036,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1216,"record_offset":110988,"parent_record_offset":110892,"ancestor_record_offset":110036,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1216,"record_offset":111044,"parent_record_offset":110892,"ancestor_record_offset":110036,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1216,"record_offset":110932,"parent_record_offset":110892,"ancestor_record_offset":110036,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1216,"record_offset":110652,"parent_record_offset":110624,"ancestor_record_offset":110036,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1216,"record_offset":110680,"parent_record_offset":110652,"ancestor_record_offset":110036,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::get_wall_attachment_height::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1216,"record_offset":110272,"parent_record_offset":110240,"ancestor_record_offset":110036,"inlinee":59493,"source_line":203,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1216,"record_offset":110868,"parent_record_offset":110844,"ancestor_record_offset":110036,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1216,"record_offset":110892,"parent_record_offset":110868,"ancestor_record_offset":110036,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1216,"record_offset":110568,"parent_record_offset":110540,"ancestor_record_offset":110036,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1216,"record_offset":110596,"parent_record_offset":110568,"ancestor_record_offset":110036,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1216,"record_offset":110720,"parent_record_offset":110568,"ancestor_record_offset":110036,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1216,"record_offset":110844,"parent_record_offset":110568,"ancestor_record_offset":110036,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01216.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::impl$0::get_wall_attachment_height::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE865
// 首条位置: {"module":1216,"inlinee":59493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1216,"record_offset":110296,"parent_record_offset":110272,"ancestor_record_offset":110272,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01216.jsonl.gz"}
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
// 完整结构及原始类型记录: public_wall_state.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::public_wall_state::PublicWallState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 15; 过程引用数: 24
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "std::collections::hash::map::Keys", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::WallId", "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "std::collections::hash::map::Iter", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: public_wall_state.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::public_wall_state::PublicWallState", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: public_wall_state.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 41; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz
// 信息 ID: 6ad59edb6b3a614499a56379
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd3fd -> 0xc9c8a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
// 信息 ID: adc0eaa8ae1c712ce6d9e6ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_on_wall_height_changed::decorator_on_wall_height_changed::closure$0 @ 0x4feabb -> 0xc9c920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x503579 -> 0xc9c920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::move_ops::calculate_move_dst @ 0x1e6653d -> 0xc9c920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3 @ 0x1e6ab7b -> 0xc9c920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
// 信息 ID: a8e50905c76d0215604c22f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_floors::construct_floors @ 0x4e5d87 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea856 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x4f460a -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xa52065 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc93836 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize @ 0xc9e2d5 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9ed2e -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0xc9ef46 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca0138 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0xca318e -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca45b9 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4d3c -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7428 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7b2e -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd063 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings @ 0x1b37d46 -> 0xc9c6f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=24952; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=452, inline_record=10652; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=452, inline_record=14072; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=17248; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::is_circle
// 信息 ID: b40f293fb0e9c82cb849e0a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x4d5626 -> 0xc9c870 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$4::call_once<tuple$<ref$<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0> @ 0x17d3701 -> 0xc9c870 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=11136; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=15352; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
// 信息 ID: 87f1401ecfc50a6c653b310e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=29388; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=369, inline_record=14280; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=10216; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=19452; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=374, inline_record=11576; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=374, inline_record=12004; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$32,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=69324; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends; module=453, inline_record=26656; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
// 信息 ID: 18407afc0aa809d00fc71045
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 67 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=24800; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs; module=362, inline_record=9840; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_floors::construct_floors; module=367, inline_record=3940; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=374, inline_record=11096; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height; module=377, inline_record=3340; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=26348; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=27560; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=24888; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
// 信息 ID: e1898fd63c56078f2407ba05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_columns @ 0x4d5fd0 -> 0xc9c840 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets @ 0x4d671b -> 0xc9c840 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fb6cd -> 0xc9c840 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501c11 -> 0xc9c840 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0xcd742a -> 0xc9c840 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x12e2edd -> 0xc9c840 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c5424f -> 0xc9c840 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate; module=401, inline_record=8648; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::closure_env$0>; module=1357, inline_record=96384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims; module=1461, inline_record=71588; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::max_y
// 信息 ID: 56ed83ba5e83aff501bfd73e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 61 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ea836 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ee81f -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ee834 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ee84e -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ee859 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ef010 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ef01b -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ef035 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ee936 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x4ee881 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f00bd -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f00e1 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1d2b -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1d84 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1e05 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x502049 -> 0xc9c740 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=19944; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=21752; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height; module=1216, inline_record=110360; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=1302, inline_record=23452; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::wall_outline::wall_outline::closure_env$3> >; module=1308, inline_record=153632; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=15384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=15988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=25052; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
// 信息 ID: 9f5ceeda9d551cf88ab22377
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcce570 -> 0xc9c7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd1483 -> 0xc9c7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad7df5 -> 0xc9c7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad9778 -> 0xc9c7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::critters_avoid_collisions @ 0x1b358c4 -> 0xc9c7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x1b36af4 -> 0xc9c7a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
// 信息 ID: 388c02f4ab2f33155ca9de98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4eaec6 -> 0xc9c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4eaf2f -> 0xc9c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccce82 -> 0xc9c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcce3ba -> 0xc9c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0xcd9139 -> 0xc9c720 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=391, inline_record=24980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=452, inline_record=10680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=452, inline_record=14100; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape; module=1216, inline_record=108456; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=17276; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::u_space
// 信息 ID: cbca687bbca3411ccc83db82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d7982 -> 0xc9c820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments @ 0xcb7b7f -> 0xc9c820 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbd7e3 -> 0xc9c820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::wall_space
// 信息 ID: 0f75ccd39b807fe0f1ffae71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 36 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500bbb -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500bc9 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500c77 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500d15 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501c67 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501cd9 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501d38 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501f92 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501ff1 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x5020d9 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x502138 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x5021c8 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::should_snap_to_corner @ 0x502a45 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x5036ff -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xcca94a -> 0xc9c800 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a2b8 -> 0xc9c800 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::impl$0::get_wall_attachment_height::closure$0
// 信息 ID: 9138c7b0a8746fe981881f19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::public_wall_state::impl$0::get_wall_attachment_height", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height; module=1216, inline_record=110272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
// 信息 ID: 481f0f2b85ff176228112ac2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::public_wall_state::impl$0::max_y", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=20004; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_outline::wall_outline; module=411, inline_record=21812; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::max_y; module=1216, inline_record=108864; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=1302, inline_record=23512; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::wall_outline::wall_outline::closure_env$3> >; module=1308, inline_record=153692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=15444; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=16048; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=2831, inline_record=25116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::impl$0::max_y_gable::closure$0
// 信息 ID: 95b9b1a469f4479ed8b0d36c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::public_wall_state::impl$0::max_y_gable", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable; module=1216, inline_record=109116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
