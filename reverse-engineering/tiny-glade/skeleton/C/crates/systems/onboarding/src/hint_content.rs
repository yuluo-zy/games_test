// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/onboarding/src/hint_content.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 582ab64f99b457cb4d3db11dce8ec2e4389bd6a0b7ee59585f3561324bdcb46e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hint_content.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3954, 3955, 3956, 3957, 3960, 3962, 3964, 3965, 3966, 3967, 3968, 3974, 3978
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use system_onboarding; // namespace candidate
//   package identities: [{"name":"system-onboarding","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::disable_snapping_hint
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=14
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20DAA
// 首条位置: {"module":3957,"inlinee":134570}
// 编译实例 RVAs: 0x287c790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23912563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162000,"raw_hex":"e88d1f93fe","size":5,"soft_fail":false,"target_va":5387249456,"va":5411162014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7970704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162000,"raw_hex":"e8706086ff","size":5,"soft_fail":false,"target_va":5403191360,"va":5411162059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3957,"record_offset":1388,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03957.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3957,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":1304,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03957.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3957,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":1304,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03957.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::focus_or_camera_binding_hint
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D96
// 首条位置: {"module":3955,"inlinee":134550}
// 编译实例 RVAs: 0x287c8a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23912835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162272,"raw_hex":"e87d1e93fe","size":5,"soft_fail":false,"target_va":5387249456,"va":5411162286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7970976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162272,"raw_hex":"e8605f86ff","size":5,"soft_fail":false,"target_va":5403191360,"va":5411162331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3955,"record_offset":5308,"parent_record_offset":5224,"ancestor_record_offset":5224,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03955.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3955,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":5224,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03955.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3955,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":5224,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03955.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::lmb_drag_binding_hint
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20DE8
// 首条位置: {"module":3967,"inlinee":134632}
// 编译实例 RVAs: 0x287c7e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23912697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162080,"raw_hex":"e8071f93fe","size":5,"soft_fail":false,"target_va":5387249440,"va":5411162132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23912659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162080,"raw_hex":"e82d1f93fe","size":5,"soft_fail":false,"target_va":5387249456,"va":5411162110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7972522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162080,"raw_hex":"e8565986ff","size":5,"soft_fail":false,"target_va":5403189648,"va":5411162165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7972089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162080,"raw_hex":"e8075b86ff","size":5,"soft_fail":false,"target_va":5403190112,"va":5411162196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287c8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t88","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411162080,"raw_hex":"e858000000","size":5,"soft_fail":false,"target_va":5411162352,"va":5411162259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3967,"record_offset":2024,"parent_record_offset":1976,"ancestor_record_offset":1976,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03967.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3967,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":1976,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03967.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::lmb_click_hint
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E1C
// 首条位置: {"module":3974,"inlinee":134684}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3974,"record_offset":8640,"parent_record_offset":8608,"ancestor_record_offset":8608,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03974.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3974,"record_offset":8664,"parent_record_offset":8640,"ancestor_record_offset":8608,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03974.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::rmb_click_hint
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E1B
// 首条位置: {"module":3974,"inlinee":134683}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3974,"record_offset":8488,"parent_record_offset":8456,"ancestor_record_offset":8456,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03974.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3974,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":8456,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03974.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::camera_mode_binding_hint
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20E2A
// 首条位置: {"module":3978,"inlinee":134698}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3978,"record_offset":1076,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":1108,"parent_record_offset":1040,"ancestor_record_offset":1040,"inlinee":108070,"source_line":111,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::camera_pan_hint
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3978,"record_offset":696,"rva":42451728,"va":5411160848}
// 编译实例 RVAs: 0x287c310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23911430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411160848,"raw_hex":"e8fa2393fe","size":5,"soft_fail":false,"target_va":5387249440,"va":5411160865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7971102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411160848,"raw_hex":"e8e25e86ff","size":5,"soft_fail":false,"target_va":5403190112,"va":5411161209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7970397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411160848,"raw_hex":"e8a36186ff","size":5,"soft_fail":false,"target_va":5403190576,"va":5411160968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3978,"record_offset":888,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":836,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_content::camera_mode_binding_hint
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_content.rs
//     evidence={"module":3978,"record_offset":1040,"parent_record_offset":696,"ancestor_record_offset":696,"inlinee":134698,"source_line":42,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3978,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":696,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3978,"record_offset":1076,"parent_record_offset":1040,"ancestor_record_offset":696,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":1108,"parent_record_offset":1040,"ancestor_record_offset":696,"inlinee":108070,"source_line":111,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::camera_rotate_hint
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3978,"record_offset":1404,"rva":42452304,"va":5411161424}
// 编译实例 RVAs: 0x287c550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23912046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161424,"raw_hex":"e8922193fe","size":5,"soft_fail":false,"target_va":5387249440,"va":5411161481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23912008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161424,"raw_hex":"e8b82193fe","size":5,"soft_fail":false,"target_va":5387249456,"va":5411161459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e1d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7973341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161424,"raw_hex":"e8235686ff","size":5,"soft_fail":false,"target_va":5403188480,"va":5411161816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7971870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161424,"raw_hex":"e8e25b86ff","size":5,"soft_fail":false,"target_va":5403189648,"va":5411161513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7971576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161424,"raw_hex":"e8085d86ff","size":5,"soft_fail":false,"target_va":5403190112,"va":5411161683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287c8f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161424,"raw_hex":"e894010000","size":5,"soft_fail":false,"target_va":5411162352,"va":5411161943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3978,"record_offset":1664,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":1552,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":1824,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":108068,"source_line":129,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_content::camera_mode_binding_hint
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_content.rs
//     evidence={"module":3978,"record_offset":1692,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":134698,"source_line":42,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3978,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":1404,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3978,"record_offset":1724,"parent_record_offset":1692,"ancestor_record_offset":1404,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":1756,"parent_record_offset":1692,"ancestor_record_offset":1404,"inlinee":108070,"source_line":111,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::camera_zoom_hint
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3978,"record_offset":1152,"rva":42452112,"va":5411161232}
// 编译实例 RVAs: 0x287c490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23911811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161232,"raw_hex":"e87d2293fe","size":5,"soft_fail":false,"target_va":5387249456,"va":5411161262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e1fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7972138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161232,"raw_hex":"e8d65a86ff","size":5,"soft_fail":false,"target_va":5403189184,"va":5411161317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7971240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161232,"raw_hex":"e8585e86ff","size":5,"soft_fail":false,"target_va":5403190112,"va":5411161347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287c8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411161232,"raw_hex":"e8a8030000","size":5,"soft_fail":false,"target_va":5411162352,"va":5411161411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":1288,"parent_record_offset":1152,"ancestor_record_offset":1152,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3978,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":1152,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::undo_redo_hint
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3978,"record_offset":272,"rva":42451328,"va":5411160448}
// 编译实例 RVAs: 0x287c180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411ae730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23911031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411160448,"raw_hex":"e8892593fe","size":5,"soft_fail":false,"target_va":5387249456,"va":5411160482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e1d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7972193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411160448,"raw_hex":"e89f5a86ff","size":5,"soft_fail":false,"target_va":5403188480,"va":5411160668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e1fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7971357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411160448,"raw_hex":"e8e35d86ff","size":5,"soft_fail":false,"target_va":5403189184,"va":5411160536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e2360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7970582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411160448,"raw_hex":"e8ea6086ff","size":5,"soft_fail":false,"target_va":5403190112,"va":5411160689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287c8f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hint_content.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411160448,"raw_hex":"e816060000","size":5,"soft_fail":false,"target_va":5411162352,"va":5411160789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3978,"record_offset":524,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":412,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":552,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":108068,"source_line":129,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3978,"record_offset":436,"parent_record_offset":412,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   [pdb_inline_transitive] system_onboarding::hint_binding::impl$4::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3978,"record_offset":612,"parent_record_offset":584,"ancestor_record_offset":272,"inlinee":108070,"source_line":111,"archive":"_pdb/C/modules/03978.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::quick_tool_palette_hint
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D8F
// 首条位置: {"module":3954,"inlinee":134543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3954,"record_offset":4712,"parent_record_offset":4628,"ancestor_record_offset":4628,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03954.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3954,"record_offset":4656,"parent_record_offset":4628,"ancestor_record_offset":4628,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03954.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3954,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4628,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03954.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_onboarding::hint_content::ui_confirm_hint
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20DC1
// 首条位置: {"module":3960,"inlinee":134593}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3960,"record_offset":1304,"parent_record_offset":1220,"ancestor_record_offset":1220,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_direct] system_onboarding::hint_binding::BindingHintDisplay::steam
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-onboarding","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/onboarding/src/hint_binding.rs
//     evidence={"module":3960,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":1220,"inlinee":108076,"source_line":180,"archive":"_pdb/C/modules/03960.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3960,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":1220,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03960.jsonl.gz"}
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
// 完整结构及原始类型记录: hint_content.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: hint_content.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: hint_content.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::camera_mode_binding_hint
// 信息 ID: f462834a8963542816c303f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_content::camera_pan_hint; module=3978, inline_record=1040; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_content::camera_rotate_hint; module=3978, inline_record=1692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::camera_pan_hint
// 信息 ID: 7aeef887418aa3a20402f4f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::camera_rotate_hint
// 信息 ID: eaaa6ed717947bc22c32f04f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::camera_zoom_hint
// 信息 ID: f1222ce958a44c005130420f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::disable_snapping_hint
// 信息 ID: 3620268cee8229cdf320ef4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [静态直接引用] system_onboarding::hint_wall_group_selection::hint_wall_group_selection @ 0x2873e12 -> 0x287c790 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_onboarding::hint_wall_group_selection::hint_wall_group_selection; module=3957, inline_record=1304; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_wall_group_selection::hint_wall_group_selection; module=3957, inline_record=1876; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_unlink_windows::hint_unlink_windows; module=3962, inline_record=1828; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_temp_eraser::hint_temp_eraser; module=3964, inline_record=536; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_disable_snapping::hint_disable_snapping; module=3965, inline_record=1500; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_terrain::hint_terrain_layering; module=3968, inline_record=1984; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_stairs::hint_stairs; module=3974, inline_record=7916; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::focus_or_camera_binding_hint
// 信息 ID: e1d34f1c281dd7a4e53abae1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_onboarding::hint_clutter_focus_edit::hint_clutter_focused_edit_hint @ 0x2102048 -> 0x287c8a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_wall_group_selection::hint_wall_group_selection @ 0x2873dde -> 0x287c8a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_onboarding::hint_deselect_tool::hint_deselect_tool; module=3955, inline_record=5224; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_focused_edit::hint_focused_edit_hint; module=3956, inline_record=4752; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_wall_group_selection::hint_wall_group_selection; module=3957, inline_record=2572; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_terrain_focus::hint_terrain_focus; module=3966, inline_record=1932; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::lmb_click_hint
// 信息 ID: 936cddf68be978b18118afa8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_stairs::hint_stairs; module=3974, inline_record=8608; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_stairs::hint_stairs; module=3974, inline_record=10004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::lmb_drag_binding_hint
// 信息 ID: 21bb6694c92d02c9d9e578ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_onboarding::hint_roof_shape::hint_roof_shape @ 0x287a918 -> 0x287c7e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_roof_shape::hint_roof_shape @ 0x287a953 -> 0x287c7e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_onboarding::hint_terrain_steepness::hint_terrain_steepness; module=3967, inline_record=1976; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_terrain_steepness::hint_terrain_steepness; module=3967, inline_record=2488; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::quick_tool_palette_hint
// 信息 ID: 92ab4bd1e3c4a059f5786d47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_quick_tool_palette::hint_quick_tool_palette; module=3954, inline_record=4628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::rmb_click_hint
// 信息 ID: d2d69931833de1c3bde8deea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::hint_stairs::hint_stairs; module=3974, inline_record=8456; 不是独立调用边。
// [PDB 内联宿主] system_onboarding::hint_stairs::hint_stairs; module=3974, inline_record=9124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::ui_confirm_hint
// 信息 ID: d7a44bfc72ea90c41e0421db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_onboarding::prompt_rollercoaster; module=3960, inline_record=1220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_onboarding::hint_content::undo_redo_hint
// 信息 ID: a1281a455f6221fcbb9631d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_onboarding::hint_undo::hint_undo @ 0x287990a -> 0x287c180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
