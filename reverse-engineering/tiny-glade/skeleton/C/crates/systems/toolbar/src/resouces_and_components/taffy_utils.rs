// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/resouces_and_components/taffy_utils.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1af0de0fed41441a8a682987d20717133332cc4228e4e0e5f0aa6d2d929ce5b8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: taffy_utils.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2949, 2952, 4855, 4856, 4858, 5048
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use taffy; // namespace candidate
//   package identities: [{"name":"taffy","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.13"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::impl$0::centered
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x238DE
// 首条位置: {"module":4855,"inlinee":145630}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B6CB
// 首条位置: {"module":2952,"inlinee":112331}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2952,"record_offset":9468,"parent_record_offset":9424,"ancestor_record_offset":9424,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02952.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_update.rs
//     evidence={"module":2952,"record_offset":9504,"parent_record_offset":9424,"ancestor_record_offset":9424,"inlinee":112341,"source_line":108,"archive":"_pdb/C/modules/02952.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_update.rs
//     evidence={"module":2952,"record_offset":9956,"parent_record_offset":9876,"ancestor_record_offset":9876,"inlinee":112318,"source_line":114,"archive":"_pdb/C/modules/02952.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::quick_tool_palette::quick_tool_palette_position::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":3660,"rva":37436864,"va":5406145984}
// 编译实例 RVAs: 0x23b3dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406145984,"raw_hex":"e860f3ffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406146043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10475434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406145984,"raw_hex":"e8aad79f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406146193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18775800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406145984,"raw_hex":"e80881e1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406146083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406145984,"raw_hex":"e8f97be1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406146018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":3884,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::quick_tool_palette::quick_tool_palette_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/quick_tool_palette.rs
//     evidence={"module":2949,"record_offset":3920,"parent_record_offset":3660,"ancestor_record_offset":3660,"inlinee":112315,"source_line":469,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::resouces_and_components::taffy_layout::impl$1::perform_layout::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":4944,"rva":37437760,"va":5406146880}
// 编译实例 RVAs: 0x23b4140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146880,"raw_hex":"e8d1efffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406146954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10474484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146880,"raw_hex":"e8f4d39f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406147143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18776746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146880,"raw_hex":"e8567de1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406147029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146880,"raw_hex":"e87278e1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406146921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":5184,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::impl$1::perform_layout::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":2949,"record_offset":5220,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":112322,"source_line":100,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$10>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":6228,"rva":37438656,"va":5406147776}
// 编译实例 RVAs: 0x23b44c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147776,"raw_hex":"e860ecffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406147835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10473650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147776,"raw_hex":"e8b2d09f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406147977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147776,"raw_hex":"e8107ae1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406147867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18778887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147776,"raw_hex":"e8f974e1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406147810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":6440,"parent_record_offset":6228,"ancestor_record_offset":6228,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$10
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2949,"record_offset":6476,"parent_record_offset":6228,"ancestor_record_offset":6228,"inlinee":112326,"source_line":248,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$15>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":3984,"rva":37437088,"va":5406146208}
// 编译实例 RVAs: 0x23b3ea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146208,"raw_hex":"e880f2ffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406146267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10475218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146208,"raw_hex":"e8d2d69f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406146409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18776016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146208,"raw_hex":"e83080e1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406146299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146208,"raw_hex":"e8197be1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406146242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":4196,"parent_record_offset":3984,"ancestor_record_offset":3984,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$15
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2949,"record_offset":4232,"parent_record_offset":3984,"ancestor_record_offset":3984,"inlinee":112316,"source_line":266,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$19>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":5292,"rva":37438032,"va":5406147152}
// 编译实例 RVAs: 0x23b4250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147152,"raw_hex":"e8d0eeffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406147211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10474274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147152,"raw_hex":"e822d39f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406147353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18776960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147152,"raw_hex":"e8807ce1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406147243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18778263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147152,"raw_hex":"e86977e1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406147186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":5504,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$19
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2949,"record_offset":5540,"parent_record_offset":5292,"ancestor_record_offset":5292,"inlinee":112323,"source_line":296,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$5>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":3348,"rva":37436656,"va":5406145776}
// 编译实例 RVAs: 0x23b3cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406145776,"raw_hex":"e830f4ffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406145835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10475650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406145776,"raw_hex":"e882d89f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406145977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18775584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406145776,"raw_hex":"e8e081e1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406145867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18776887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406145776,"raw_hex":"e8c97ce1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406145810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":3560,"parent_record_offset":3348,"ancestor_record_offset":3348,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2949,"record_offset":3596,"parent_record_offset":3348,"ancestor_record_offset":3348,"inlinee":112314,"source_line":230,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":5916,"rva":37438448,"va":5406147568}
// 编译实例 RVAs: 0x23b43f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147568,"raw_hex":"e830edffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406147627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10473858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147568,"raw_hex":"e882d19f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406147769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147568,"raw_hex":"e8e07ae1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406147659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18778679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147568,"raw_hex":"e8c975e1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406147602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":6128,"parent_record_offset":5916,"ancestor_record_offset":5916,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::undo_redo_alignment::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2949,"record_offset":6164,"parent_record_offset":5916,"ancestor_record_offset":5916,"inlinee":112325,"source_line":318,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":6540,"rva":37438864,"va":5406147984}
// 编译实例 RVAs: 0x23b4590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147984,"raw_hex":"e890ebffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406148043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10473442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147984,"raw_hex":"e8e2cf9f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406148185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147984,"raw_hex":"e84079e1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406148075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18779095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147984,"raw_hex":"e82974e1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406148018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":6752,"parent_record_offset":6540,"ancestor_record_offset":6540,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::undo_redo_alignment::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2949,"record_offset":6788,"parent_record_offset":6540,"ancestor_record_offset":6540,"inlinee":112327,"source_line":329,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$5>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":4632,"rva":37437552,"va":5406146672}
// 编译实例 RVAs: 0x23b4070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146672,"raw_hex":"e8b0f0ffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406146731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10474754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146672,"raw_hex":"e802d59f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406146873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18776480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146672,"raw_hex":"e8607ee1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406146763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146672,"raw_hex":"e84979e1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406146706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":4844,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::undo_redo_alignment::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2949,"record_offset":4880,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":112320,"source_line":335,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$9>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":5604,"rva":37438240,"va":5406147360}
// 编译实例 RVAs: 0x23b4320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147360,"raw_hex":"e800eeffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406147419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10474066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147360,"raw_hex":"e852d29f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406147561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147360,"raw_hex":"e8b07be1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406147451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18778471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406147360,"raw_hex":"e89976e1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406147394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":5816,"parent_record_offset":5604,"ancestor_record_offset":5604,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::undo_redo_alignment::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":2949,"record_offset":5852,"parent_record_offset":5604,"ancestor_record_offset":5604,"inlinee":112324,"source_line":351,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2949,"record_offset":4296,"rva":37437296,"va":5406146416}
// 编译实例 RVAs: 0x23b3f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1423b3160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146416,"raw_hex":"e8abf1ffff","size":5,"soft_fail":false,"target_va":5406142816,"va":5406146480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10474966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146416,"raw_hex":"e8d6d59f00","size":5,"soft_fail":false,"target_va":5416621632,"va":5406146661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18776266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146416,"raw_hex":"e8367fe1fe","size":5,"soft_fail":false,"target_va":5387370288,"va":5406146549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18777532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406146416,"raw_hex":"e8447ae1fe","size":5,"soft_fail":false,"target_va":5387368928,"va":5406146455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2949,"record_offset":4528,"parent_record_offset":4296,"ancestor_record_offset":4296,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02949.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_update.rs
//     evidence={"module":2949,"record_offset":4564,"parent_record_offset":4296,"ancestor_record_offset":4296,"inlinee":112318,"source_line":114,"archive":"_pdb/C/modules/02949.jsonl.gz"}
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
// 完整结构及原始类型记录: taffy_utils.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: taffy_utils.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: taffy_utils.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::impl$0::centered
// 信息 ID: 0c92dc0868aecc6200b97887
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_contextual_menu; module=4855, inline_record=5588; 不是独立调用边。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::add_toolbar_item; module=4855, inline_record=10820; 不是独立调用边。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::toolbar_startup::setup_toolbar::closure_env$0>; module=4856, inline_record=1828; 不是独立调用边。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0>; module=4856, inline_record=4948; 不是独立调用边。
// [PDB 内联宿主] toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout::setup; module=4858, inline_record=1292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style
// 信息 ID: 6a1ee0935fa6ceefe0e6659a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::toolbar_update::toolbar_button_size_and_padding_update; module=2952, inline_record=9424; 不是独立调用边。
// [PDB 内联宿主] toolbar::toolbar_update::toolbar_button_size_and_padding_update; module=2952, inline_record=9876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::quick_tool_palette::quick_tool_palette_position::closure_env$0>
// 信息 ID: 738f5fbf813910962fcf1ba7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::quick_tool_palette::quick_tool_palette_position::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::quick_tool_palette::quick_tool_palette_position @ 0x11d5991 -> 0x23b3dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::quick_tool_palette::quick_tool_palette_position @ 0x11d59d0 -> 0x23b3dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::resouces_and_components::taffy_layout::impl$1::perform_layout::closure_env$1>
// 信息 ID: fdf93f17c4f80300d135a966
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::resouces_and_components::taffy_layout::impl$1::perform_layout::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::taffy_update::taffy_finalize_element_size @ 0x2961c1c -> 0x23b4140 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::resouces_and_components::taffy_layout::TaffyLayout::perform_layout @ 0x296a60b -> 0x23b4140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$10>
// 信息 ID: 65799ccd7424886d12bc88df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$10"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962743 -> 0x23b44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x296295d -> 0x23b44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962a58 -> 0x23b44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962c5d -> 0x23b44c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_alignment @ 0x296387d -> 0x23b44c0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$15>
// 信息 ID: 7c638855240870d914969f13
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$15"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962984 -> 0x23b3ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_alignment @ 0x296385e -> 0x23b3ea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$19>
// 信息 ID: 3ecb4e7fd47e8162c8d632f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$19"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x296272e -> 0x23b4250 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962754 -> 0x23b4250 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962a47 -> 0x23b4250 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962a69 -> 0x23b4250 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962c84 -> 0x23b4250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$5>
// 信息 ID: 890c8ae991ee5b1d8660cfe3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$5"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$0>
// 信息 ID: 34764e1fe6ba7cfd11ebd3e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::undo_redo_alignment::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$3>
// 信息 ID: f3b3ec33265983327b3e9b33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::undo_redo_alignment::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$5>
// 信息 ID: 01ee2423a9b83da1a26b0ac3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::undo_redo_alignment::closure_env$5"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962930 -> 0x23b4070 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962c27 -> 0x23b4070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$9>
// 信息 ID: f257ffbc0fde77a5986d0d45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::undo_redo_alignment::closure_env$9"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x296271c -> 0x23b4320 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2962a35 -> 0x23b4320 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_alignment @ 0x296384d -> 0x23b4320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$2>
// 信息 ID: 3ad0d9f6d0e680a5cf688739
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_update::toolbar_button_size_and_padding_update @ 0x23b6a46 -> 0x23b3f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
