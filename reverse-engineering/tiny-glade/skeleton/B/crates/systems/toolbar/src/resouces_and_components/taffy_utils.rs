// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/resouces_and_components/taffy_utils.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1af0de0fed41441a8a682987d20717133332cc4228e4e0e5f0aa6d2d929ce5b8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: taffy_utils.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1187, 1234, 2780, 2835, 3945, 4526
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
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1E4
// 首条位置: {"module":1187,"inlinee":61924}
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
// IPI IDs: 0x18847
// 首条位置: {"module":2835,"inlinee":100423}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2835,"record_offset":8384,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_update.rs
//     evidence={"module":2835,"record_offset":8424,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":100425,"source_line":104,"archive":"_pdb/B/modules/02835.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_update.rs
//     evidence={"module":2835,"record_offset":8876,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":100426,"source_line":110,"archive":"_pdb/B/modules/02835.jsonl.gz"}
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
// 首条位置: {"module":3945,"record_offset":5264,"rva":37183776,"va":5405892896}
// 编译实例 RVAs: 0x2376120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892896,"raw_hex":"e861eeffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405892970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5649716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892896,"raw_hex":"e834355600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405893159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20506282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892896,"raw_hex":"e85619c7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405893045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20505758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892896,"raw_hex":"e8621bc7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405892937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":5504,"parent_record_offset":5264,"ancestor_record_offset":5264,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::impl$1::perform_layout::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3945,"record_offset":5540,"parent_record_offset":5264,"ancestor_record_offset":5264,"inlinee":122998,"source_line":100,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3945,"record_offset":3368,"rva":37182480,"va":5405891600}
// 编译实例 RVAs: 0x2375c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891600,"raw_hex":"e880f3ffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405891659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5651074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891600,"raw_hex":"e8823a5600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405891801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20504928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891600,"raw_hex":"e8a01ec7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405891691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20504455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891600,"raw_hex":"e87920c7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405891634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":3580,"parent_record_offset":3368,"ancestor_record_offset":3368,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":3945,"record_offset":3616,"parent_record_offset":3368,"ancestor_record_offset":3368,"inlinee":122992,"source_line":192,"archive":"_pdb/B/modules/03945.jsonl.gz"}
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
// 首条位置: {"module":3945,"record_offset":3056,"rva":37182272,"va":5405891392}
// 编译实例 RVAs: 0x2375b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891392,"raw_hex":"e850f4ffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405891451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5651282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891392,"raw_hex":"e8523b5600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405891593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20504720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891392,"raw_hex":"e8701fc7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405891483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20504247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891392,"raw_hex":"e84921c7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405891426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":3268,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$10
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":3945,"record_offset":3304,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":122991,"source_line":229,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$11>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3945,"record_offset":3680,"rva":37182688,"va":5405891808}
// 编译实例 RVAs: 0x2375ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891808,"raw_hex":"e8b0f2ffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405891867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5650866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891808,"raw_hex":"e8b2395600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405892009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20505136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891808,"raw_hex":"e8d01dc7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405891899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20504663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405891808,"raw_hex":"e8a91fc7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405891842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":3892,"parent_record_offset":3680,"ancestor_record_offset":3680,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$11
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":3945,"record_offset":3928,"parent_record_offset":3680,"ancestor_record_offset":3680,"inlinee":122993,"source_line":232,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$12>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3945,"record_offset":4952,"rva":37183568,"va":5405892688}
// 编译实例 RVAs: 0x2376050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892688,"raw_hex":"e840efffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405892747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5649986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892688,"raw_hex":"e842365600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405892889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20506016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892688,"raw_hex":"e8601ac7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405892779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20505543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892688,"raw_hex":"e8391cc7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405892722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":5164,"parent_record_offset":4952,"ancestor_record_offset":4952,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$12
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":3945,"record_offset":5200,"parent_record_offset":4952,"ancestor_record_offset":4952,"inlinee":122997,"source_line":237,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$13>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3945,"record_offset":5924,"rva":37184256,"va":5405893376}
// 编译实例 RVAs: 0x2376300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405893376,"raw_hex":"e890ecffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405893435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5649298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405893376,"raw_hex":"e892335600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405893577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20506704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405893376,"raw_hex":"e8b017c7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405893467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20506231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405893376,"raw_hex":"e88919c7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405893410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":6136,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$13
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":3945,"record_offset":6172,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":123000,"source_line":241,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3945,"record_offset":4328,"rva":37183152,"va":5405892272}
// 编译实例 RVAs: 0x2375eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892272,"raw_hex":"e8e0f0ffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405892331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5650402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892272,"raw_hex":"e8e2375600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405892473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20505600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892272,"raw_hex":"e8001cc7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405892363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20505127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892272,"raw_hex":"e8d91dc7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405892306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":4540,"parent_record_offset":4328,"ancestor_record_offset":4328,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::toolbar_alignment::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":3945,"record_offset":4576,"parent_record_offset":4328,"ancestor_record_offset":4328,"inlinee":122994,"source_line":199,"archive":"_pdb/B/modules/03945.jsonl.gz"}
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
// 首条位置: {"module":3945,"record_offset":5612,"rva":37184048,"va":5405893168}
// 编译实例 RVAs: 0x2376230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405893168,"raw_hex":"e860edffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405893227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5649506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405893168,"raw_hex":"e862345600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405893369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20506496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405893168,"raw_hex":"e88018c7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405893259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20506023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405893168,"raw_hex":"e8591ac7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405893202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":5824,"parent_record_offset":5612,"ancestor_record_offset":5612,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::undo_redo_alignment::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":3945,"record_offset":5860,"parent_record_offset":5612,"ancestor_record_offset":5612,"inlinee":122999,"source_line":299,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3945,"record_offset":4640,"rva":37183360,"va":5405892480}
// 编译实例 RVAs: 0x2375f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892480,"raw_hex":"e810f0ffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405892539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5650194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892480,"raw_hex":"e812375600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405892681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20505808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892480,"raw_hex":"e8301bc7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405892571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20505335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892480,"raw_hex":"e8091dc7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405892514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":4852,"parent_record_offset":4640,"ancestor_record_offset":4640,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_systems::undo_redo_alignment::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_systems.rs
//     evidence={"module":3945,"record_offset":4888,"parent_record_offset":4640,"ancestor_record_offset":4640,"inlinee":122996,"source_line":302,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3945,"record_offset":3992,"rva":37182896,"va":5405892016}
// 编译实例 RVAs: 0x2375db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142374fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892016,"raw_hex":"e8dbf1ffff","size":5,"soft_fail":false,"target_va":5405888464,"va":5405892080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5650614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892016,"raw_hex":"e8b6385600","size":5,"soft_fail":false,"target_va":5411542880,"va":5405892261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::set_style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20505386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892016,"raw_hex":"e8d61cc7fe","size":5,"soft_fail":false,"target_va":5385386768,"va":5405892149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20504876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405892016,"raw_hex":"e8d41ec7fe","size":5,"soft_fail":false,"target_va":5385387184,"va":5405892055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3945,"record_offset":4224,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03945.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_update.rs
//     evidence={"module":3945,"record_offset":4260,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":100425,"source_line":104,"archive":"_pdb/B/modules/03945.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::impl$0::centered
// 信息 ID: a098b100f28a6da3a0748479
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout::setup; module=1187, inline_record=1444; 不是独立调用边。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::add_toolbar_item; module=2780, inline_record=8316; 不是独立调用边。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::toolbar_startup::setup_toolbar::closure_env$0>; module=4526, inline_record=1172; 不是独立调用边。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0>; module=4526, inline_record=4760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style
// 信息 ID: 6ca9394827351d42993e8cc9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::toolbar_update::toolbar_button_size_and_padding_update; module=2835, inline_record=8340; 不是独立调用边。
// [PDB 内联宿主] toolbar::toolbar_update::toolbar_button_size_and_padding_update; module=2835, inline_record=8796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::resouces_and_components::taffy_layout::impl$1::perform_layout::closure_env$1>
// 信息 ID: 03d0a3cef22849859ce463e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::resouces_and_components::taffy_layout::impl$1::perform_layout::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::resouces_and_components::taffy_layout::TaffyLayout::perform_layout @ 0x1e4c7d5 -> 0x2376120 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::taffy_update::taffy_finalize_element_size @ 0x234097b -> 0x2376120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$0>
// 信息 ID: d3d426cf138a82b66e83da43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$10>
// 信息 ID: 2c6e2403ee63ba605dfe6170
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$10"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2344682 -> 0x2375b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2344850 -> 0x2375b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_alignment @ 0x2344920 -> 0x2375b40 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$11>
// 信息 ID: bb825a3aabd005cec24f811b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$11"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x234466e -> 0x2375ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2344692 -> 0x2375ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2344880 -> 0x2375ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$12>
// 信息 ID: e4d0cca7875d60b79881fda4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$12"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x234465d -> 0x2376050 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_alignment @ 0x23448ef -> 0x2376050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$13>
// 信息 ID: 5c2e9ee51fd737d6e195c5f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$13"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::undo_redo_alignment @ 0x2344901 -> 0x2376300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$2>
// 信息 ID: a2c2f5cef16b11051b304c21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::toolbar_alignment::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_systems::toolbar_alignment @ 0x2344827 -> 0x2375eb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$0>
// 信息 ID: 72130597b280e4d41f043430
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::undo_redo_alignment::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$1>
// 信息 ID: 88dc71769bb181799a8e6176
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_systems::undo_redo_alignment::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$1>
// 信息 ID: 24ce2c9379ed3413860831ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::resouces_and_components::taffy_utils::taffy_mut_style -> ["toolbar::toolbar_update::toolbar_button_size_and_padding_update::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_update::toolbar_button_size_and_padding_update @ 0x1eaf326 -> 0x2375db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
