// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/walls/public_wall_state.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d7fe5e903d8bea9329457bf533459d645fc5bcb84fc078f16b15fb75ab49546b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: public_wall_state.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 390, 391, 392, 394, 396, 397, 399, 402, 413, 420, 429, 441, 442, 445, 450, 451, 453, 456, 471, 496, 497, 499, 512, 513, 515, 519, 524, 527, 532, 533, 550, 551, 553, 558, 559, 563, 565, 566, 567, 578, 583, 584, 1177, 1413, 1434, 1436, 1443, 1449, 1451, 1476, 1516, 1522, 1585, 1610, 1640, 1734, 3098, 3723, 3756, 3764, 3767, 3805, 3883, 3887
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
// 记录形式: inline_instance, inlinee_declaration; instances=123
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F5B
// 首条位置: {"module":390,"inlinee":24411}
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
// 记录形式: inline_instance, inlinee_declaration; instances=39
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F5F
// 首条位置: {"module":390,"inlinee":24415}
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
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x742D
// 首条位置: {"module":519,"inlinee":29741}
// 编译实例 RVAs: 0xb2a5d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31124760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380416976,"raw_hex":"e818edda01","size":5,"soft_fail":false,"target_va":5411541776,"va":5380417011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":519,"record_offset":94560,"parent_record_offset":94504,"ancestor_record_offset":94504,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":519,"record_offset":94532,"parent_record_offset":94504,"ancestor_record_offset":94504,"inlinee":29742,"source_line":131,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x742E
// 首条位置: {"module":519,"inlinee":29742}
// 编译实例 RVAs: 0xb2a600
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=33
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7363
// 首条位置: {"module":515,"inlinee":29539}
// 编译实例 RVAs: 0xb2a620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31124767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417056,"raw_hex":"e81fedda01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380417132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":515,"record_offset":24852,"parent_record_offset":24824,"ancestor_record_offset":24824,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":515,"record_offset":24908,"parent_record_offset":24884,"ancestor_record_offset":24824,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00515.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":515,"record_offset":24884,"parent_record_offset":24852,"ancestor_record_offset":24824,"inlinee":29547,"source_line":147,"archive":"_pdb/B/modules/00515.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=28
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x736B
// 首条位置: {"module":515,"inlinee":29547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":515,"record_offset":24908,"parent_record_offset":24884,"ancestor_record_offset":24884,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00515.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
// 记录形式: compiled_procedure; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":140388,"rva":11708032,"va":5380417152}
// 编译实例 RVAs: 0xb2a680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31124671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417152,"raw_hex":"e8bfecda01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380417228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":583,"record_offset":140552,"parent_record_offset":140388,"ancestor_record_offset":140388,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":583,"record_offset":140600,"parent_record_offset":140580,"ancestor_record_offset":140388,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y_gable::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":583,"record_offset":140580,"parent_record_offset":140552,"ancestor_record_offset":140388,"inlinee":33250,"source_line":152,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::impl$0::max_y_gable::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81E2
// 首条位置: {"module":583,"inlinee":33250}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":583,"record_offset":140600,"parent_record_offset":140580,"ancestor_record_offset":140580,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":140640,"rva":11708128,"va":5380417248}
// 编译实例 RVAs: 0xb2a6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31124567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417248,"raw_hex":"e857ecda01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380417332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":583,"record_offset":140804,"parent_record_offset":140640,"ancestor_record_offset":140640,"inlinee":29539,"source_line":146,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":583,"record_offset":140876,"parent_record_offset":140856,"ancestor_record_offset":140640,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":583,"record_offset":140828,"parent_record_offset":140804,"ancestor_record_offset":140640,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":583,"record_offset":140856,"parent_record_offset":140828,"ancestor_record_offset":140640,"inlinee":29547,"source_line":147,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::wall_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":140920,"rva":11708224,"va":5380417344}
// 编译实例 RVAs: 0xb2a740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":583,"record_offset":141084,"parent_record_offset":140920,"ancestor_record_offset":140920,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::wall_space_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":141112,"rva":11708256,"va":5380417376}
// 编译实例 RVAs: 0xb2a760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::u_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":141276,"rva":11708272,"va":5380417392}
// 编译实例 RVAs: 0xb2a770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":583,"record_offset":141436,"parent_record_offset":141276,"ancestor_record_offset":141276,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 184
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EC6
// 首条位置: {"module":567,"inlinee":32454}
// 编译实例 RVAs: 0xb2a790
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
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75B0
// 首条位置: {"module":527,"inlinee":30128}
// 编译实例 RVAs: 0xb2a7d0
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
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":141776,"rva":11708416,"va":5380417536}
// 编译实例 RVAs: 0xb2a800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1505515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417536,"raw_hex":"e8ebf81600","size":5,"soft_fail":false,"target_va":5381923136,"va":5380417616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1502652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417536,"raw_hex":"e8bced1600","size":5,"soft_fail":false,"target_va":5381920256,"va":5380417599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
// 记录形式: compiled_procedure; instances=1
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":583,"record_offset":141944,"rva":11708560,"va":5380417680}
// 编译实例 RVAs: 0xb2a890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141467ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: public_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9688032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417680,"raw_hex":"e8e0d39300","size":5,"soft_fail":false,"target_va":5390105824,"va":5380417787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: public_wall_state.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9696421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417680,"raw_hex":"e8a5f49300","size":5,"soft_fail":false,"target_va":5390114304,"va":5380417878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31124085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417680,"raw_hex":"e875eada01","size":5,"soft_fail":false,"target_va":5411542064,"va":5380417974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31124226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417680,"raw_hex":"e802ebda01","size":5,"soft_fail":false,"target_va":5411542296,"va":5380418065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31123846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380417680,"raw_hex":"e886e9da01","size":5,"soft_fail":false,"target_va":5411541904,"va":5380418053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":583,"record_offset":142148,"parent_record_offset":141944,"ancestor_record_offset":141944,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":583,"record_offset":142240,"parent_record_offset":141944,"ancestor_record_offset":141944,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":583,"record_offset":142268,"parent_record_offset":141944,"ancestor_record_offset":141944,"inlinee":29539,"source_line":146,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":583,"record_offset":142448,"parent_record_offset":141944,"ancestor_record_offset":141944,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":583,"record_offset":142392,"parent_record_offset":141944,"ancestor_record_offset":141944,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":583,"record_offset":142296,"parent_record_offset":141944,"ancestor_record_offset":141944,"inlinee":24559,"source_line":191,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":583,"record_offset":143052,"parent_record_offset":141944,"ancestor_record_offset":141944,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":583,"record_offset":142532,"parent_record_offset":142476,"ancestor_record_offset":141944,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":583,"record_offset":142416,"parent_record_offset":142392,"ancestor_record_offset":141944,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":583,"record_offset":142360,"parent_record_offset":142296,"ancestor_record_offset":141944,"inlinee":24560,"source_line":547,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":583,"record_offset":142332,"parent_record_offset":142296,"ancestor_record_offset":141944,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":583,"record_offset":142204,"parent_record_offset":142180,"ancestor_record_offset":141944,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":583,"record_offset":142976,"parent_record_offset":142948,"ancestor_record_offset":141944,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":583,"record_offset":142832,"parent_record_offset":142800,"ancestor_record_offset":141944,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":583,"record_offset":143280,"parent_record_offset":143052,"ancestor_record_offset":141944,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":583,"record_offset":142560,"parent_record_offset":142532,"ancestor_record_offset":141944,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":583,"record_offset":142588,"parent_record_offset":142560,"ancestor_record_offset":141944,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::impl$0::get_wall_attachment_height::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":583,"record_offset":142180,"parent_record_offset":142148,"ancestor_record_offset":141944,"inlinee":33252,"source_line":232,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":583,"record_offset":142776,"parent_record_offset":142752,"ancestor_record_offset":141944,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":583,"record_offset":142800,"parent_record_offset":142776,"ancestor_record_offset":141944,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":583,"record_offset":142948,"parent_record_offset":142776,"ancestor_record_offset":141944,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":583,"record_offset":142476,"parent_record_offset":142448,"ancestor_record_offset":141944,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":583,"record_offset":142504,"parent_record_offset":142476,"ancestor_record_offset":141944,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":583,"record_offset":142628,"parent_record_offset":142476,"ancestor_record_offset":141944,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":583,"record_offset":142752,"parent_record_offset":142476,"ancestor_record_offset":141944,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00583.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::public_wall_state::impl$0::get_wall_attachment_height::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81E4
// 首条位置: {"module":583,"inlinee":33252}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":583,"record_offset":142204,"parent_record_offset":142180,"ancestor_record_offset":142180,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00583.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 17; 过程引用数: 25
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "std::collections::hash::map::Keys", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> >", "constructor": "core::iter::adapters::copied::Copied", "arguments": ["std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::WallId", "core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "constructor": "std::collections::hash::map::Iter", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>", "country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::public_wall_state::PublicWallState", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [已有逆向证据 B / native_observed_partial_layout; not full original struct declaration] country_core::resources::walls::public_wall_state::PublicWallState
// name: "country_core::resources::walls::public_wall_state::PublicWallState"
// baseline: "B"
// status: "native_observed_partial_layout; not full original struct declaration"
// alignment: "unknown"
// complete_size: "unknown"
// unobserved_fields: "unknown"
// original_field_names: "unknown unless explicitly established"
// evidence: [{"path": "reconstruction/building/wall_state.rs", "sha256": "3ac4555841032f8347e1d28da60be7260eead44ae7408c8de265169ce269cf4d"}, {"path": "evidence/building-gather/findings.md", "sha256": "dc42e7dc122c1fac5186624d3a3b78e27d2f06b4cb4f84fdae0f13e13e90d0a9"}, {"path": "evidence/building-state/B-spatial-verification.json", "sha256": "f5d7b8cadf45a2d8f85e16a8d5453f3fea9349075cb59d09aa5815b54b2064a4"}]
// observed_storage_span: 672
// span_status: "B observed storage projection, not recovered Rust ABI"
// [观察字段；未命名者 original_name=unknown] {"offset": 16, "width": 8, "type": "u64 read", "original_name": "unknown", "observed_semantics": "raw Curve2 point count"}
// [观察字段；未命名者 original_name=unknown] {"offset": 48, "width": 4, "type": "f32", "original_name": "unknown", "observed_semantics": "raw Curve2 length"}
// [观察字段；未命名者 original_name=unknown] {"offset": 264, "width": 1, "type": "u8 bit test; full field type unknown", "original_name": "unknown", "observed_semantics": "max_y cache presence bit 0"}
// [观察字段；未命名者 original_name=unknown] {"offset": 268, "width": 4, "type": "f32", "original_name": "unknown", "observed_semantics": "max_y cache value"}
// [观察字段；未命名者 original_name=unknown] {"offset": 272, "width": 1, "type": "u8 bit test; full field type unknown", "original_name": "unknown", "observed_semantics": "gable max_y cache presence bit 0"}
// [观察字段；未命名者 original_name=unknown] {"offset": 276, "width": 4, "type": "f32", "original_name": "unknown", "observed_semantics": "gable max_y cache value"}
// [观察字段；未命名者 original_name=unknown] {"offset": 600, "width": 28, "type": "unknown; raw shape record", "original_name": "unknown", "observed_semantics": "shape tag at first four bytes; payload incomplete"}
// [观察字段；未命名者 original_name=unknown] {"offset": 652, "width": 4, "type": "f32", "original_name": "unknown", "observed_semantics": "published raw Curve2 length"}
// [观察字段；未命名者 original_name=unknown] {"offset": 665, "width": 1, "type": "u8 read; original enum unknown", "original_name": "unknown", "observed_semantics": "style; half-timber consumer checks >= 2"}
// ------------------------------------------------------------------
// [已有逆向证据 B / native_observed_partial_layout; not full original struct declaration] PublicWallState publication source bindings (partial)
// name: "PublicWallState publication source bindings (partial)"
// baseline: "B"
// status: "native_observed_partial_layout; not full original struct declaration"
// alignment: "unknown"
// complete_size: "unknown"
// unobserved_fields: "unknown"
// original_field_names: "unknown unless explicitly established"
// evidence: [{"path": "evidence/building-production-authority/B-publication-source-fields.json", "sha256": "c8c4152d73f4c39292fd40e9f69876ec411185968aa4027245c58545a0010094"}]
// evidence_scope: "Original publisher instruction source-load/scratch-publication-copy binding only; no Rust candidate/fullpublisher claim"
// limitations: ["Creation defaults", "Public map insertion/storage copy", "Original source field semantic naming", "A/C source/public offsets", "Full candidatepublication chain"]
// [观察字段；未命名者 original_name=unknown] {"offset": 660, "original_name": "unknown", "type": "unknown", "observed_semantics": "observed arch-global selection flag, original name/default not asserted", "raw_source_offset": 100, "binding": {"raw_offset": "0x64", "public_offset": "0x294", "name": "observed arch-global selection flag, original name/default not asserted", "range_load": ["0x140a8d81e", "0x140a8d84b"], "range_store": ["0x140a8d904", "0x140a8d928"]}}
// [观察字段；未命名者 original_name=unknown] {"offset": 662, "original_name": "unknown", "type": "unknown", "observed_semantics": "observed special flag", "raw_source_offset": 101, "binding": {"raw_offset": "0x65", "public_offset": "0x296", "name": "observed special flag", "range_load": ["0x140a8ce58", "0x140a8ce6f"], "range_store": ["0x140a8db74", "0x140a8db81"]}}
// [观察字段；未命名者 original_name=unknown] {"offset": 661, "original_name": "unknown", "type": "unknown", "observed_semantics": "ClutterVersion observed byte", "raw_source_offset": 102, "binding": {"raw_offset": "0x66", "public_offset": "0x295", "name": "ClutterVersion observed byte", "range_load": ["0x140a8d81e", "0x140a8d84b"], "range_store": ["0x140a8d904", "0x140a8d928"]}}
// [观察字段；未命名者 original_name=unknown] {"offset": 648, "original_name": "unknown", "type": "unknown", "observed_semantics": "observed path scalar, original type/name not asserted", "raw_source_offset": 92, "binding": {"raw_offset": "0x5c", "public_offset": "0x288", "name": "observed path scalar, original type/name not asserted", "range_load": ["0x140a8d81e", "0x140a8d84b"], "range_store": ["0x140a8d904", "0x140a8d928"]}}
// [观察字段；未命名者 original_name=unknown] {"offset": 656, "original_name": "unknown", "type": "unknown", "observed_semantics": "unknown scalar bits", "raw_source_offset": 96, "binding": {"raw_offset": "0x60", "public_offset": "0x290", "load": "0x140a8d831", "store": "0x140a8db88", "semantic_name": "unknown scalar bits"}}
// [观察字段；未命名者 original_name=unknown] {"offset": 664, "original_name": "unknown", "type": "unknown", "observed_semantics": "unknown byte", "raw_source_offset": 103, "binding": {"raw_offset": "0x67", "public_offset": "0x298", "load": "0x140a8d841", "store": "0x140a8db90", "semantic_name": "unknown byte"}}
// [观察字段；未命名者 original_name=unknown] {"offset": 665, "original_name": "unknown", "type": "unknown", "observed_semantics": "style", "raw_source_offset": 104, "binding": {"raw_offset": "0x68", "public_offset": "0x299", "load": "0x140a8d846", "store": "0x140a8db6c", "semantic_name": "style"}}
// [观察字段；未命名者 original_name=unknown] {"offset": 666, "original_name": "unknown", "type": "unknown", "observed_semantics": "raw metadata; new_wall depends on DEV_OVERRIDE_IS_DEMO_MODE", "raw_source_offset": 105, "binding": {"raw_offset": "0x69", "public_offset": "0x29a", "load": "0x140a8d846", "store": "0x140a8db6c", "semantic_name": "raw metadata; new_wall depends on DEV_OVERRIDE_IS_DEMO_MODE"}}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: public_wall_state.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-decorator-info/B-anchor-consumers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::public_wall_state::PublicWallState", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-gather/B-gable-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::public_wall_state::PublicWallState", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-integrate/B-core/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::public_wall_state::PublicWallState", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-runtime/B-cache/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::public_wall_state::PublicWallState", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/B-platform-coordinates/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::public_wall_state::PublicWallState", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::public_wall_state::PublicWallState", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/pdb-structure/type-layout-coverage.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::public_wall_state::PublicWallState", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: public_wall_state.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 49; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::reader::EventReader<country_core::game_profile::SaveGameProfileCmd>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::game_profile::SaveGameProfileCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::game_profile::SaveGameProfileCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::corner_pos_and_normal_xz
// 信息 ID: dd6a3fe3f361f4803cc1f60b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x8159f4 -> 0xb2a800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y
// 信息 ID: d570b64c4c05550f6b54b704
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xe4b6 -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xe53c -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1d8f6 -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1d97c -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::find_stair_placement_candidate @ 0x7a7f2a -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::find_stair_placement_candidate @ 0x7a815f -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7acc50 -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_change_stair_height::ui_change_stair_height @ 0x7ae00d -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance @ 0x7b09e3 -> 0xb2a6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height
// 信息 ID: 30f6b18ef4d52978210db9f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs @ 0x7a0996 -> 0xb2a890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_on_wall_height_changed::decorator_on_wall_height_changed::closure$0 @ 0x81ba62 -> 0xb2a890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::move_decorators_following_anchors::move_decorators_following_anchors @ 0x81d238 -> 0xb2a890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3 @ 0x127a57b -> 0xb2a890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
// 信息 ID: 870b5a975acacb7aeee81eb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x74fb4c -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x75876f -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_floors::construct_floors @ 0x766698 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xf92178 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217e51 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x1244595 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x12494c6 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize @ 0x124b205 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124bcee -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x124bf06 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x124d0d0 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars @ 0x124fe7e -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x12511c3 -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0x1251aed -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x12544db -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0x1254d9a -> 0xb2a5d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=519, inline_record=94504; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=1640, inline_record=51296; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=1734, inline_record=139424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=1734, inline_record=142956; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::is_circle
// 信息 ID: d5afc92b50eb4913d9dbd492
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x788466 -> 0xb2a7d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$4::call_once<tuple$<ref$<glam::f32::vec2::Vec2> >,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure$0::closure_env$0> @ 0x12ac9f1 -> 0xb2a7d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle; module=527, inline_record=78792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=1640, inline_record=46436; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=1640, inline_record=49456; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
// 信息 ID: b120311f0497b7d81e2e2552
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=390, inline_record=9820; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=390, inline_record=10308; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=392, inline_record=14620; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=9244; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=19904; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=396, inline_record=29968; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=397, inline_record=444; 不是独立调用边。
// [PDB 内联宿主] system_decorator::blueprint::show_max_wall_y; module=471, inline_record=33032; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
// 信息 ID: 06c0e97fc07a58f44b412acc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 70 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=390, inline_record=9296; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs; module=391, inline_record=8968; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=396, inline_record=13396; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=396, inline_record=25588; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::profile::create_stone_arch_profile; module=397, inline_record=7148; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_floors::construct_floors; module=399, inline_record=4212; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=402, inline_record=11208; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height; module=413, inline_record=3384; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
// 信息 ID: c336fa50df65330dacb4a8c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_columns @ 0x788f92 -> 0xb2a790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets @ 0x789c18 -> 0xb2a790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x78bc9a -> 0xb2a790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f6ce4 -> 0xb2a790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0x122228b -> 0xb2a790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x1a38aab -> 0xb2a790 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims; module=567, inline_record=51548; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate; module=567, inline_record=70812; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to::closure_env$0>; module=578, inline_record=146220; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::max_y
// 信息 ID: d2e3266d6e2863fcf1713fab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 71 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xea7f -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1dedf -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x75874f -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x75c1ca -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75dd2c -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75dd41 -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75dd5b -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75dd68 -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75e4c9 -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75e4d4 -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75e4f0 -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75de33 -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::split_wall_into_sides @ 0x75dd8e -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x775927 -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x77594b -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7776db -> 0xb2a620 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline; module=515, inline_record=24824; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=519, inline_record=60280; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=519, inline_record=60400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=519, inline_record=60460; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle; module=527, inline_record=86316; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=550, inline_record=39108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=550, inline_record=39688; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=550, inline_record=53848; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
// 信息 ID: 1b6c6fd291458b407ca360b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x1219090 -> 0xb2a680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121bf8d -> 0xb2a680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d94fe -> 0xb2a680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22dae24 -> 0xb2a680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::critters_avoid_collisions @ 0x231878f -> 0xb2a680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x2319d3a -> 0xb2a680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
// 信息 ID: f03acac3829bb6b5c83e5444
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xe491 -> 0xb2a600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1d8d1 -> 0xb2a600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x7590c1 -> 0xb2a600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x75912e -> 0xb2a600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x75c18e -> 0xb2a600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1217c61 -> 0xb2a600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x1218ecc -> 0xb2a600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0x1223d99 -> 0xb2a600 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::shape_placement_params; module=519, inline_record=94532; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape; module=583, inline_record=139920; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle; module=1640, inline_record=51324; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=1734, inline_record=139452; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=1734, inline_record=142984; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::u_space
// 信息 ID: 3fbbd4bfecee62c4bdab9b5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x75fed2 -> 0xb2a770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760220 -> 0xb2a770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76e337 -> 0xb2a770 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2> @ 0x11ff9c2 -> 0xb2a770 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1243d40 -> 0xb2a770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::wall_space
// 信息 ID: 27e033975a4c245337e62022
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 30 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0xea92 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x1def2 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x7c2e7b -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x7c2ed4 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones @ 0x7eb655 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::get_gothic_hat_bones @ 0x7eb685 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ed6f4 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ecb50 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ecba0 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f44e5 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x7f452b -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f75f3 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f7638 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::flag::generate_flags @ 0x80034f -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs::walk_shape @ 0x809d5e -> 0xb2a740 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs::find_alignment_preferences @ 0x80c1e7 -> 0xb2a740 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::PublicWallState::wall_space_length
// 信息 ID: 1518bac29d8a3061f1689764
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121ce90 -> 0xb2a760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x121db17 -> 0xb2a760 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x12445e4 -> 0xb2a760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::impl$0::get_wall_attachment_height::closure$0
// 信息 ID: 2532a09ff9f85f10b7ff56ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::public_wall_state::impl$0::get_wall_attachment_height", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_attachment_height; module=583, inline_record=142180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::impl$0::max_y::closure$0
// 信息 ID: e82895b4d43683df6c176e58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::public_wall_state::impl$0::max_y", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::structure_outline::structure_outline; module=515, inline_record=24884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_shapes::ui_move_shapes; module=519, inline_record=60336; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle; module=527, inline_record=86384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_focus_mode_shape_height::ui_focus_mode_shape_height; module=550, inline_record=53896; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=553, inline_record=71008; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=566, inline_record=83912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims; module=567, inline_record=62592; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::do_ui_edit_rectangle_rotate; module=567, inline_record=84944; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::public_wall_state::impl$0::max_y_gable::closure$0
// 信息 ID: bcdc07b76a63764e40217b14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::public_wall_state::impl$0::max_y_gable", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable; module=583, inline_record=140580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
