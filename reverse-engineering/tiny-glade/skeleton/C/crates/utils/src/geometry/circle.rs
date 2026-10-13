// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/geometry/circle.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 69db16956f57159dc3851baf9117729b45f441da5596b9ae2f4bf915ec88cd71
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 76, 81, 228, 551, 561, 567, 570, 574, 584, 609, 763, 764, 1545, 1600, 1643, 1682, 1762, 1774, 1795, 2186, 2741, 3631, 3779, 3799, 4675, 4677, 4988
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97AD
// 首条位置: {"module":609,"inlinee":38829}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":404,"rva":14594448,"va":5383303568}
// 编译实例 RVAs: 0xdeb190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1520491","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383303568,"raw_hex":"e96b331700","size":5,"soft_fail":false,"target_va":5384824080,"va":5383303584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":71176,"rva":28263472,"va":5396972592}
// 编译实例 RVAs: 0x1af4430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15029
// 首条位置: {"module":1795,"inlinee":86057}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":27960,"parent_record_offset":27936,"ancestor_record_offset":27936,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":27984,"parent_record_offset":27960,"ancestor_record_offset":27936,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":28008,"parent_record_offset":27984,"ancestor_record_offset":27936,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15011
// 首条位置: {"module":1774,"inlinee":86033}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15606
// 首条位置: {"module":1795,"inlinee":87558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":27704,"parent_record_offset":27628,"ancestor_record_offset":27628,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":28384,"parent_record_offset":28348,"ancestor_record_offset":27628,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":28456,"parent_record_offset":28420,"ancestor_record_offset":27628,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":28420,"parent_record_offset":28384,"ancestor_record_offset":27628,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":28580,"parent_record_offset":28544,"ancestor_record_offset":27628,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":28544,"parent_record_offset":27704,"ancestor_record_offset":27628,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":28348,"parent_record_offset":28312,"ancestor_record_offset":27628,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":28312,"parent_record_offset":28276,"ancestor_record_offset":27628,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":28276,"parent_record_offset":27704,"ancestor_record_offset":27628,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":27984,"parent_record_offset":27960,"ancestor_record_offset":27628,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":28008,"parent_record_offset":27984,"ancestor_record_offset":27628,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":27960,"parent_record_offset":27936,"ancestor_record_offset":27628,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":27900,"parent_record_offset":27704,"ancestor_record_offset":27628,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":27844,"parent_record_offset":27704,"ancestor_record_offset":27628,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1795,"record_offset":27936,"parent_record_offset":27900,"ancestor_record_offset":27628,"inlinee":86057,"source_line":16,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1795,"record_offset":27868,"parent_record_offset":27844,"ancestor_record_offset":27628,"inlinee":86033,"source_line":16,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":260,"rva":14594416,"va":5383303536}
// 编译实例 RVAs: 0xdeb170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t1520523","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383303536,"raw_hex":"e98b331700","size":5,"soft_fail":false,"target_va":5384824080,"va":5383303552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15604
// 首条位置: {"module":1795,"inlinee":87556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1795,"record_offset":29484,"parent_record_offset":27196,"ancestor_record_offset":27196,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":30104,"parent_record_offset":27196,"ancestor_record_offset":27196,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":30900,"parent_record_offset":27196,"ancestor_record_offset":27196,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":27236,"parent_record_offset":27196,"ancestor_record_offset":27196,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":29516,"parent_record_offset":27196,"ancestor_record_offset":27196,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":28384,"parent_record_offset":28348,"ancestor_record_offset":27196,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":28456,"parent_record_offset":28420,"ancestor_record_offset":27196,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":28420,"parent_record_offset":28384,"ancestor_record_offset":27196,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":28580,"parent_record_offset":28544,"ancestor_record_offset":27196,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":28544,"parent_record_offset":27704,"ancestor_record_offset":27196,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":28348,"parent_record_offset":28312,"ancestor_record_offset":27196,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":28312,"parent_record_offset":28276,"ancestor_record_offset":27196,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":28276,"parent_record_offset":27704,"ancestor_record_offset":27196,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":27984,"parent_record_offset":27960,"ancestor_record_offset":27196,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1795,"record_offset":30752,"parent_record_offset":30640,"ancestor_record_offset":27196,"inlinee":7711,"source_line":1627,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":30692,"parent_record_offset":30668,"ancestor_record_offset":27196,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":30668,"parent_record_offset":30640,"ancestor_record_offset":27196,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":30716,"parent_record_offset":30692,"ancestor_record_offset":27196,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1795,"record_offset":30004,"parent_record_offset":29980,"ancestor_record_offset":27196,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":27496,"parent_record_offset":27452,"ancestor_record_offset":27196,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":29376,"parent_record_offset":27340,"ancestor_record_offset":27196,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":28008,"parent_record_offset":27984,"ancestor_record_offset":27196,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1795,"record_offset":29956,"parent_record_offset":29928,"ancestor_record_offset":27196,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1795,"record_offset":29980,"parent_record_offset":29956,"ancestor_record_offset":27196,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":27452,"parent_record_offset":27408,"ancestor_record_offset":27196,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":27960,"parent_record_offset":27936,"ancestor_record_offset":27196,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1795,"record_offset":29628,"parent_record_offset":29600,"ancestor_record_offset":27196,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":30568,"parent_record_offset":30536,"ancestor_record_offset":27196,"inlinee":31975,"source_line":2023,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":27408,"parent_record_offset":27340,"ancestor_record_offset":27196,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":30044,"parent_record_offset":29884,"ancestor_record_offset":27196,"inlinee":17577,"source_line":2482,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":29928,"parent_record_offset":29884,"ancestor_record_offset":27196,"inlinee":17576,"source_line":2469,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":30536,"parent_record_offset":30504,"ancestor_record_offset":27196,"inlinee":31974,"source_line":2178,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":27704,"parent_record_offset":27628,"ancestor_record_offset":27196,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":29684,"parent_record_offset":29656,"ancestor_record_offset":27196,"inlinee":17629,"source_line":2310,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":29656,"parent_record_offset":29628,"ancestor_record_offset":27196,"inlinee":87550,"source_line":2327,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":27340,"parent_record_offset":27288,"ancestor_record_offset":27196,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":29572,"parent_record_offset":29544,"ancestor_record_offset":27196,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::visit_content_seq_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":29884,"parent_record_offset":29684,"ancestor_record_offset":27196,"inlinee":17574,"source_line":2043,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":27288,"parent_record_offset":27236,"ancestor_record_offset":27196,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":29544,"parent_record_offset":29516,"ancestor_record_offset":27196,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":27900,"parent_record_offset":27704,"ancestor_record_offset":27196,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":27844,"parent_record_offset":27704,"ancestor_record_offset":27196,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":27552,"parent_record_offset":27340,"ancestor_record_offset":27196,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":30504,"parent_record_offset":30472,"ancestor_record_offset":27196,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":30640,"parent_record_offset":30568,"ancestor_record_offset":27196,"inlinee":7710,"source_line":225,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":30844,"parent_record_offset":30568,"ancestor_record_offset":27196,"inlinee":17567,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":30816,"parent_record_offset":30568,"ancestor_record_offset":27196,"inlinee":17565,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":30784,"parent_record_offset":30568,"ancestor_record_offset":27196,"inlinee":7709,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":30268,"parent_record_offset":30244,"ancestor_record_offset":27196,"inlinee":33382,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1795,"record_offset":27936,"parent_record_offset":27900,"ancestor_record_offset":27196,"inlinee":86057,"source_line":16,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1795,"record_offset":27868,"parent_record_offset":27844,"ancestor_record_offset":27196,"inlinee":86033,"source_line":16,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1795,"record_offset":27628,"parent_record_offset":27552,"ancestor_record_offset":27196,"inlinee":87558,"source_line":16,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15607
// 首条位置: {"module":1795,"inlinee":87559}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":31116,"parent_record_offset":31072,"ancestor_record_offset":31072,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":31464,"parent_record_offset":31440,"ancestor_record_offset":31072,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":31072,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":31072,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":31440,"parent_record_offset":31416,"ancestor_record_offset":31072,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":31416,"parent_record_offset":31392,"ancestor_record_offset":31072,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":31392,"parent_record_offset":31360,"ancestor_record_offset":31072,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1795,"record_offset":32348,"parent_record_offset":32236,"ancestor_record_offset":31072,"inlinee":7711,"source_line":1627,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":32288,"parent_record_offset":32264,"ancestor_record_offset":31072,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":32264,"parent_record_offset":32236,"ancestor_record_offset":31072,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":32312,"parent_record_offset":32288,"ancestor_record_offset":31072,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1795,"record_offset":31680,"parent_record_offset":31656,"ancestor_record_offset":31072,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":32700,"parent_record_offset":32640,"ancestor_record_offset":31072,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":31244,"parent_record_offset":31220,"ancestor_record_offset":31072,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1795,"record_offset":31768,"parent_record_offset":31180,"ancestor_record_offset":31072,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1795,"record_offset":31632,"parent_record_offset":31604,"ancestor_record_offset":31072,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1795,"record_offset":31656,"parent_record_offset":31632,"ancestor_record_offset":31072,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":31220,"parent_record_offset":31180,"ancestor_record_offset":31072,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] glam::features::impl_serde::f32::impl$13::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/features/impl_serde.rs
//     evidence={"module":1795,"record_offset":31304,"parent_record_offset":31276,"ancestor_record_offset":31072,"inlinee":47685,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":32128,"parent_record_offset":32084,"ancestor_record_offset":31072,"inlinee":31975,"source_line":2023,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":31720,"parent_record_offset":31560,"ancestor_record_offset":31072,"inlinee":17577,"source_line":2482,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::SeqRefDeserializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":31604,"parent_record_offset":31560,"ancestor_record_offset":31072,"inlinee":17576,"source_line":2469,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":32084,"parent_record_offset":32040,"ancestor_record_offset":31072,"inlinee":31974,"source_line":2178,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":31360,"parent_record_offset":31332,"ancestor_record_offset":31072,"inlinee":17629,"source_line":2310,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_tuple_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":31332,"parent_record_offset":31304,"ancestor_record_offset":31072,"inlinee":87550,"source_line":2327,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":31180,"parent_record_offset":31148,"ancestor_record_offset":31072,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::visit_content_seq_ref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":31560,"parent_record_offset":31360,"ancestor_record_offset":31072,"inlinee":17574,"source_line":2043,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":31148,"parent_record_offset":31116,"ancestor_record_offset":31072,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":31276,"parent_record_offset":31180,"ancestor_record_offset":31072,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":32040,"parent_record_offset":31996,"ancestor_record_offset":31072,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":32236,"parent_record_offset":32128,"ancestor_record_offset":31072,"inlinee":7710,"source_line":225,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":32496,"parent_record_offset":32128,"ancestor_record_offset":31072,"inlinee":17567,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":32440,"parent_record_offset":32128,"ancestor_record_offset":31072,"inlinee":17565,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":32208,"parent_record_offset":32128,"ancestor_record_offset":31072,"inlinee":17563,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":32468,"parent_record_offset":32128,"ancestor_record_offset":31072,"inlinee":17566,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":32380,"parent_record_offset":32128,"ancestor_record_offset":31072,"inlinee":17564,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":32408,"parent_record_offset":32128,"ancestor_record_offset":31072,"inlinee":7709,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":32756,"parent_record_offset":32732,"ancestor_record_offset":31072,"inlinee":33382,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":76,"record_offset":25840,"rva":2392320,"va":5371101440}
// 编译实例 RVAs: 0x248100, 0x1af5370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2214082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371101440,"raw_hex":"e8c2c82100","size":5,"soft_fail":false,"target_va":5373315680,"va":5371101593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140466530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2221015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371101440,"raw_hex":"e8d7e32100","size":5,"soft_fail":false,"target_va":5373322544,"va":5371101524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140467400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2224631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371101440,"raw_hex":"e8f7f12100","size":5,"soft_fail":false,"target_va":5373326336,"va":5371101700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b55e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2358937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396976496,"raw_hex":"e86701dcff","size":5,"soft_fail":false,"target_va":5394617824,"va":5396976756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2341721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396976496,"raw_hex":"e8a744dcff","size":5,"soft_fail":false,"target_va":5394634864,"va":5396976580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2339982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396976496,"raw_hex":"e8724bdcff","size":5,"soft_fail":false,"target_va":5394636672,"va":5396976649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45251289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371101440,"raw_hex":"e8d97ab202","size":5,"soft_fail":false,"target_va":5416353040,"va":5371101746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t45456173","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371101440,"raw_hex":"e92d9bb502","size":5,"soft_fail":false,"target_va":5416557744,"va":5371101566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":76,"record_offset":26204,"parent_record_offset":25840,"ancestor_record_offset":25840,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":76,"record_offset":26808,"parent_record_offset":25840,"ancestor_record_offset":25840,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":76,"record_offset":26748,"parent_record_offset":25840,"ancestor_record_offset":25840,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::is_default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":76,"record_offset":26112,"parent_record_offset":25840,"ancestor_record_offset":25840,"inlinee":10598,"source_line":27,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":76,"record_offset":26600,"parent_record_offset":26568,"ancestor_record_offset":25840,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":76,"record_offset":26468,"parent_record_offset":26440,"ancestor_record_offset":25840,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":76,"record_offset":26528,"parent_record_offset":26496,"ancestor_record_offset":25840,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":76,"record_offset":26648,"parent_record_offset":26624,"ancestor_record_offset":25840,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":76,"record_offset":26624,"parent_record_offset":26600,"ancestor_record_offset":25840,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":76,"record_offset":26496,"parent_record_offset":26468,"ancestor_record_offset":25840,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":26400,"parent_record_offset":26372,"ancestor_record_offset":25840,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":26568,"parent_record_offset":26400,"ancestor_record_offset":25840,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":26344,"parent_record_offset":26316,"ancestor_record_offset":25840,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":26688,"parent_record_offset":26400,"ancestor_record_offset":25840,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":26440,"parent_record_offset":26400,"ancestor_record_offset":25840,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":76,"record_offset":26372,"parent_record_offset":26344,"ancestor_record_offset":25840,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":26140,"parent_record_offset":26112,"ancestor_record_offset":25840,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":76,"record_offset":26260,"parent_record_offset":26232,"ancestor_record_offset":25840,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":76,"record_offset":26868,"parent_record_offset":26840,"ancestor_record_offset":25840,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":76,"record_offset":26232,"parent_record_offset":26204,"ancestor_record_offset":25840,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":76,"record_offset":26840,"parent_record_offset":26808,"ancestor_record_offset":25840,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":76,"record_offset":26316,"parent_record_offset":26288,"ancestor_record_offset":25840,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":76,"record_offset":26288,"parent_record_offset":26260,"ancestor_record_offset":25840,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$7::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":76,"record_offset":26168,"parent_record_offset":26140,"ancestor_record_offset":25840,"inlinee":10600,"source_line":7,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1774,"record_offset":82160,"rva":28267040,"va":5396976160}
// 编译实例 RVAs: 0x1af5220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13748594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396976160,"raw_hex":"e88e362eff","size":5,"soft_fail":false,"target_va":5383227792,"va":5396976381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141af4c60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396976160,"raw_hex":"e8a2f9ffff","size":5,"soft_fail":false,"target_va":5396974688,"va":5396976313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b82a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t579370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396976160,"raw_hex":"e82ad70800","size":5,"soft_fail":false,"target_va":5397555792,"va":5396976417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b84570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t586440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396976160,"raw_hex":"e8c8f20800","size":5,"soft_fail":false,"target_va":5397562736,"va":5396976291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b87af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t600172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396976160,"raw_hex":"e86c280900","size":5,"soft_fail":false,"target_va":5397576432,"va":5396976255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1774,"record_offset":82424,"parent_record_offset":82160,"ancestor_record_offset":82160,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::is_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":1774,"record_offset":82332,"parent_record_offset":82160,"ancestor_record_offset":82160,"inlinee":10598,"source_line":27,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1774,"record_offset":82360,"parent_record_offset":82332,"ancestor_record_offset":82160,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1774,"record_offset":82388,"parent_record_offset":82360,"ancestor_record_offset":82160,"inlinee":10600,"source_line":7,"archive":"_pdb/C/modules/01774.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$4::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E65
// 首条位置: {"module":574,"record_offset":88256,"rva":12079344,"va":5380788464}
// 编译实例 RVAs: 0xb850f0, 0x1ac0540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field3_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4032972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380788464,"raw_hex":"e8cc893d00","size":5,"soft_fail":false,"target_va":5384821600,"va":5380788623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89A6
// 首条位置: {"module":561,"inlinee":35238}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":32324,"parent_record_offset":32296,"ancestor_record_offset":32296,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":3779,"record_offset":4740,"parent_record_offset":4680,"ancestor_record_offset":4680,"inlinee":10600,"source_line":7,"archive":"_pdb/C/modules/03779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::is_default
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2966
// 首条位置: {"module":76,"inlinee":10598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":26140,"parent_record_offset":26112,"ancestor_record_offset":26112,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$7::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":76,"record_offset":26168,"parent_record_offset":26140,"ancestor_record_offset":26112,"inlinee":10600,"source_line":7,"archive":"_pdb/C/modules/00076.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":6448,"rva":14596800,"va":5383305920}
// 编译实例 RVAs: 0xdebac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::new_rotated
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF9
// 首条位置: {"module":763,"inlinee":49145}
// 编译实例 RVAs: 0xdeb2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::area
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":6740,"rva":14596880,"va":5383306000}
// 编译实例 RVAs: 0xdebb10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":6864,"parent_record_offset":6740,"ancestor_record_offset":6740,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":6884,"parent_record_offset":6864,"ancestor_record_offset":6740,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::yaw_radians
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x862B
// 首条位置: {"module":551,"inlinee":34347}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_pos_at_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFFE
// 首条位置: {"module":763,"record_offset":1352,"rva":14594816,"va":5383303936}
// 编译实例 RVAs: 0xdeb300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33753642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383303936,"raw_hex":"e82a0a0302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383304049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33753829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383303936,"raw_hex":"e8e50a0302","size":5,"soft_fail":false,"target_va":5417057840,"va":5383304006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33753966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383303936,"raw_hex":"e86e0b0302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383304061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":1640,"parent_record_offset":1352,"ancestor_record_offset":1352,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":1728,"parent_record_offset":1352,"ancestor_record_offset":1352,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":1492,"parent_record_offset":1352,"ancestor_record_offset":1352,"inlinee":49138,"source_line":80,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":1668,"parent_record_offset":1640,"ancestor_record_offset":1352,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":1756,"parent_record_offset":1728,"ancestor_record_offset":1352,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":1580,"parent_record_offset":1492,"ancestor_record_offset":1352,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":1608,"parent_record_offset":1492,"ancestor_record_offset":1352,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":1524,"parent_record_offset":1492,"ancestor_record_offset":1352,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1352,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_tangent_at_u
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":3580,"rva":14595840,"va":5383304960}
// 编译实例 RVAs: 0xdeb700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33752639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304960,"raw_hex":"e83f060302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383305052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33752820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304960,"raw_hex":"e8f4060302","size":5,"soft_fail":false,"target_va":5417057840,"va":5383305015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33752964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304960,"raw_hex":"e884070302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383305063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":3864,"parent_record_offset":3580,"ancestor_record_offset":3580,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":3720,"parent_record_offset":3580,"ancestor_record_offset":3580,"inlinee":49138,"source_line":80,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":3804,"parent_record_offset":3720,"ancestor_record_offset":3580,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":3832,"parent_record_offset":3720,"ancestor_record_offset":3580,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":3748,"parent_record_offset":3720,"ancestor_record_offset":3580,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":3580,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_closest_u_from_pos
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":4788,"rva":14596240,"va":5383305360}
// 编译实例 RVAs: 0xdeb890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bc10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33751775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383305360,"raw_hex":"e8df020302","size":5,"soft_fail":false,"target_va":5417057296,"va":5383305516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33752067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383305360,"raw_hex":"e803040302","size":5,"soft_fail":false,"target_va":5417057600,"va":5383305528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33752281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383305360,"raw_hex":"e8d9040302","size":5,"soft_fail":false,"target_va":5417057840,"va":5383305554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33316493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383305360,"raw_hex":"e88d5efc01","size":5,"soft_fail":false,"target_va":5416622208,"va":5383305710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33316479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383305360,"raw_hex":"e87f5efc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5383305756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":5004,"parent_record_offset":4788,"ancestor_record_offset":4788,"inlinee":27634,"source_line":546,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":4952,"parent_record_offset":4788,"ancestor_record_offset":4788,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::cartesian_to_polar
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":763,"record_offset":5288,"parent_record_offset":4788,"ancestor_record_offset":4788,"inlinee":48885,"source_line":418,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":5376,"parent_record_offset":4788,"ancestor_record_offset":4788,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":763,"record_offset":5564,"parent_record_offset":4788,"ancestor_record_offset":4788,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":763,"record_offset":5436,"parent_record_offset":4788,"ancestor_record_offset":4788,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":763,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":4788,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":763,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":4788,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":5472,"parent_record_offset":5436,"ancestor_record_offset":4788,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":4788,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":5220,"parent_record_offset":5192,"ancestor_record_offset":4788,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":4788,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":763,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":4788,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":763,"record_offset":5596,"parent_record_offset":5564,"ancestor_record_offset":4788,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":5092,"parent_record_offset":5060,"ancestor_record_offset":4788,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":4788,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":5036,"parent_record_offset":5004,"ancestor_record_offset":4788,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":5192,"parent_record_offset":5004,"ancestor_record_offset":4788,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":5344,"parent_record_offset":5288,"ancestor_record_offset":4788,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":4788,"inlinee":14783,"source_line":675,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":5124,"parent_record_offset":5092,"ancestor_record_offset":4788,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":4788,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":763,"record_offset":5500,"parent_record_offset":5436,"ancestor_record_offset":4788,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::get_normal_at_u
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF2
// 首条位置: {"module":763,"inlinee":49138}
// 编译实例 RVAs: 0xdeb620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33752863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304736,"raw_hex":"e81f070302","size":5,"soft_fail":false,"target_va":5417057696,"va":5383304828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33753044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304736,"raw_hex":"e8d4070302","size":5,"soft_fail":false,"target_va":5417057840,"va":5383304791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33753188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304736,"raw_hex":"e864080302","size":5,"soft_fail":false,"target_va":5417058032,"va":5383304839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":1580,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":1608,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":1524,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1492,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::perimeter
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 111, 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFEF
// 首条位置: {"module":763,"inlinee":49135}
// 编译实例 RVAs: 0xdebf70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::intersect_ray
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFFF
// 首条位置: {"module":763,"record_offset":1820,"rva":14595008,"va":5383304128}
// 编译实例 RVAs: 0xdeb3c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":1956,"parent_record_offset":1820,"ancestor_record_offset":1820,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":1984,"parent_record_offset":1820,"ancestor_record_offset":1820,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":1820,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::intersect_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":3896,"rva":14595984,"va":5383305104}
// 编译实例 RVAs: 0xdeb790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":4224,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":4052,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":4364,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":4420,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":4308,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":4252,"parent_record_offset":3896,"ancestor_record_offset":3896,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":3896,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":3896,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":3896,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":4100,"parent_record_offset":4072,"ancestor_record_offset":3896,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":4124,"parent_record_offset":4072,"ancestor_record_offset":3896,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":4072,"parent_record_offset":4052,"ancestor_record_offset":3896,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":4156,"parent_record_offset":4124,"ancestor_record_offset":3896,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::is_point_inside
// 记录形式: compiled_procedure; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":2964,"rva":14595760,"va":5383304880}
// 编译实例 RVAs: 0xdeb6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":3100,"parent_record_offset":2964,"ancestor_record_offset":2964,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2964,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":2964,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":3172,"parent_record_offset":3120,"ancestor_record_offset":2964,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":3120,"parent_record_offset":3100,"ancestor_record_offset":2964,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":3200,"parent_record_offset":3172,"ancestor_record_offset":2964,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::closest_point_on_perimeter
// 记录形式: compiled_procedure; instances=1
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":5852,"rva":14596704,"va":5383305824}
// 编译实例 RVAs: 0xdeba60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":6060,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":6292,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":6368,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":6012,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":5852,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":5852,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":6232,"parent_record_offset":6080,"ancestor_record_offset":5852,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":6032,"parent_record_offset":6012,"ancestor_record_offset":5852,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":6140,"parent_record_offset":6112,"ancestor_record_offset":5852,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":6112,"parent_record_offset":6080,"ancestor_record_offset":5852,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":6080,"parent_record_offset":6060,"ancestor_record_offset":5852,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":6168,"parent_record_offset":6140,"ancestor_record_offset":5852,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points2_ptcount
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF0
// 首条位置: {"module":763,"inlinee":49136}
// 编译实例 RVAs: 0xdeb870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":763,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":800,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":763,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":800,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$0::as_points2_ptcount::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B0F
// 首条位置: {"module":81,"inlinee":11023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":81,"record_offset":198652,"parent_record_offset":198620,"ancestor_record_offset":198620,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":2186,"record_offset":11688,"parent_record_offset":11508,"ancestor_record_offset":11508,"inlinee":49150,"source_line":59,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":81,"record_offset":198736,"parent_record_offset":198708,"ancestor_record_offset":198620,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":81,"record_offset":198760,"parent_record_offset":198736,"ancestor_record_offset":198620,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2186,"record_offset":11896,"parent_record_offset":11868,"ancestor_record_offset":11508,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2186,"record_offset":11984,"parent_record_offset":11956,"ancestor_record_offset":11508,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":81,"record_offset":198708,"parent_record_offset":198684,"ancestor_record_offset":198620,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":81,"record_offset":198684,"parent_record_offset":198652,"ancestor_record_offset":198620,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2186,"record_offset":11868,"parent_record_offset":11688,"ancestor_record_offset":11508,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2186,"record_offset":11956,"parent_record_offset":11688,"ancestor_record_offset":11508,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2186,"record_offset":11808,"parent_record_offset":11716,"ancestor_record_offset":11508,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2186,"record_offset":11836,"parent_record_offset":11716,"ancestor_record_offset":11508,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":2186,"record_offset":11748,"parent_record_offset":11716,"ancestor_record_offset":11508,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2186,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":11508,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":2186,"record_offset":11716,"parent_record_offset":11688,"ancestor_record_offset":11508,"inlinee":49138,"source_line":80,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF1
// 首条位置: {"module":763,"inlinee":49137}
// 编译实例 RVAs: 0xdeb1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33754030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383303600,"raw_hex":"e8ae0b0302","size":5,"soft_fail":false,"target_va":5417057664,"va":5383303629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":763,"record_offset":772,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":716,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2_ptcount
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":800,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":49136,"source_line":195,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":692,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":49135,"source_line":111,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":548,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":763,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":548,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":763,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":548,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_points3
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF5
// 首条位置: {"module":763,"inlinee":49141}
// 编译实例 RVAs: 0xdeb240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33753878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383303744,"raw_hex":"e8160b0302","size":5,"soft_fail":false,"target_va":5417057664,"va":5383303781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":763,"record_offset":1168,"parent_record_offset":888,"ancestor_record_offset":888,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":1024,"parent_record_offset":888,"ancestor_record_offset":888,"inlinee":49137,"source_line":202,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":763,"record_offset":1136,"parent_record_offset":1024,"ancestor_record_offset":888,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":888,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":763,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":888,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":1080,"parent_record_offset":1024,"ancestor_record_offset":888,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":888,"inlinee":49135,"source_line":111,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve2
// 记录形式: compiled_procedure; instances=1
// 源码行: 212
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":7612,"rva":14597392,"va":5383306512}
// 编译实例 RVAs: 0xdebd10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9731053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306512,"raw_hex":"e813846bff","size":5,"soft_fail":false,"target_va":5373575632,"va":5383306680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e357e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17078835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306512,"raw_hex":"e8339a0401","size":5,"soft_fail":false,"target_va":5400385504,"va":5383306664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33751114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306512,"raw_hex":"e84a000302","size":5,"soft_fail":false,"target_va":5417057664,"va":5383306545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33314840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306512,"raw_hex":"e81858fc01","size":5,"soft_fail":false,"target_va":5416621632,"va":5383306787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":763,"record_offset":7892,"parent_record_offset":7612,"ancestor_record_offset":7612,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":763,"record_offset":7976,"parent_record_offset":7612,"ancestor_record_offset":7612,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":7748,"parent_record_offset":7612,"ancestor_record_offset":7612,"inlinee":49137,"source_line":202,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":763,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":7612,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":763,"record_offset":7860,"parent_record_offset":7748,"ancestor_record_offset":7612,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":7612,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":763,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":7612,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":763,"record_offset":8004,"parent_record_offset":7976,"ancestor_record_offset":7612,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":7804,"parent_record_offset":7748,"ancestor_record_offset":7612,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":7776,"parent_record_offset":7748,"ancestor_record_offset":7612,"inlinee":49135,"source_line":111,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve3
// 记录形式: compiled_procedure; instances=1
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":8048,"rva":14597680,"va":5383306800}
// 编译实例 RVAs: 0xdebe30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141499460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7001458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306800,"raw_hex":"e872d56a00","size":5,"soft_fail":false,"target_va":5390308448,"va":5383306985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e35490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17077682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306800,"raw_hex":"e8b2950401","size":5,"soft_fail":false,"target_va":5400384656,"va":5383306969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33750815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306800,"raw_hex":"e81fff0202","size":5,"soft_fail":false,"target_va":5417057664,"va":5383306844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33314527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306800,"raw_hex":"e8df56fc01","size":5,"soft_fail":false,"target_va":5416621632,"va":5383307100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":763,"record_offset":8356,"parent_record_offset":8048,"ancestor_record_offset":8048,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":763,"record_offset":8440,"parent_record_offset":8048,"ancestor_record_offset":8048,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::as_points3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":8184,"parent_record_offset":8048,"ancestor_record_offset":8048,"inlinee":49141,"source_line":207,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":763,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":8048,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":763,"record_offset":8320,"parent_record_offset":8208,"ancestor_record_offset":8048,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":8048,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":763,"record_offset":8404,"parent_record_offset":8380,"ancestor_record_offset":8048,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":763,"record_offset":8468,"parent_record_offset":8440,"ancestor_record_offset":8048,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":8264,"parent_record_offset":8208,"ancestor_record_offset":8048,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::as_points2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":8048,"inlinee":49137,"source_line":202,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":8048,"inlinee":49135,"source_line":111,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::as_curve2_range
// 记录形式: compiled_procedure; instances=1
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":2284,"rva":14595264,"va":5383304384}
// 编译实例 RVAs: 0xdeb4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9728976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304384,"raw_hex":"e8308c6bff","size":5,"soft_fail":false,"target_va":5373575632,"va":5383304603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e34a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17077488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304384,"raw_hex":"e8f0940401","size":5,"soft_fail":false,"target_va":5400382080,"va":5383304587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33753203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304384,"raw_hex":"e873080302","size":5,"soft_fail":false,"target_va":5417057664,"va":5383304456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33316901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383304384,"raw_hex":"e82560fc01","size":5,"soft_fail":false,"target_va":5416621632,"va":5383304726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":763,"record_offset":2520,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":763,"record_offset":2548,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":2464,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":763,"record_offset":2632,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::perimeter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":2436,"parent_record_offset":2284,"ancestor_record_offset":2284,"inlinee":49135,"source_line":111,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":763,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":2284,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":2284,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":763,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":2284,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":763,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":2284,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::impl$0::as_curve2_range::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1829C
// 首条位置: {"module":2186,"inlinee":98972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":2186,"record_offset":6968,"parent_record_offset":6932,"ancestor_record_offset":6932,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":2186,"record_offset":7080,"parent_record_offset":6932,"ancestor_record_offset":6932,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":2186,"record_offset":7228,"parent_record_offset":6932,"ancestor_record_offset":6932,"inlinee":49150,"source_line":59,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2186,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":6932,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2186,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":6932,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2186,"record_offset":7048,"parent_record_offset":6968,"ancestor_record_offset":6932,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2186,"record_offset":7020,"parent_record_offset":6968,"ancestor_record_offset":6932,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2186,"record_offset":7484,"parent_record_offset":7460,"ancestor_record_offset":6932,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2186,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":6932,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2186,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":6932,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2186,"record_offset":7112,"parent_record_offset":7080,"ancestor_record_offset":6932,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2186,"record_offset":7404,"parent_record_offset":7228,"ancestor_record_offset":6932,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2186,"record_offset":7460,"parent_record_offset":7228,"ancestor_record_offset":6932,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2186,"record_offset":7344,"parent_record_offset":7256,"ancestor_record_offset":6932,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2186,"record_offset":7372,"parent_record_offset":7256,"ancestor_record_offset":6932,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":2186,"record_offset":7284,"parent_record_offset":7256,"ancestor_record_offset":6932,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2186,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":6932,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":2186,"record_offset":7256,"parent_record_offset":7228,"ancestor_record_offset":6932,"inlinee":49138,"source_line":80,"archive":"_pdb/C/modules/02186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::with_expanded_diameter
// 记录形式: compiled_procedure; instances=1
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":5716,"rva":14596656,"va":5383305776}
// 编译实例 RVAs: 0xdeba30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::contains_point
// 记录形式: compiled_procedure; instances=1
// 源码行: 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":2044,"rva":14595216,"va":5383304336}
// 编译实例 RVAs: 0xdeb490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":2232,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":2184,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":2204,"parent_record_offset":2184,"ancestor_record_offset":2044,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":2252,"parent_record_offset":2232,"ancestor_record_offset":2044,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::signed_distance
// 记录形式: compiled_procedure; instances=1
// 源码行: 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":3272,"rva":14595808,"va":5383304928}
// 编译实例 RVAs: 0xdeb6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":3408,"parent_record_offset":3272,"ancestor_record_offset":3272,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":763,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":3272,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":3272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":3480,"parent_record_offset":3428,"ancestor_record_offset":3272,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":3428,"parent_record_offset":3408,"ancestor_record_offset":3272,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":763,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":3272,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::aabb
// 记录形式: compiled_procedure; instances=1
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":6564,"rva":14596832,"va":5383305952}
// 编译实例 RVAs: 0xdebae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":6688,"parent_record_offset":6564,"ancestor_record_offset":6564,"inlinee":8901,"source_line":1579,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":6708,"parent_record_offset":6688,"ancestor_record_offset":6564,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::circle::Circle2d::lerp
// 记录形式: compiled_procedure; instances=1
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":763,"record_offset":6916,"rva":14596912,"va":5383306032}
// 编译实例 RVAs: 0xdebb30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33751686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383306032,"raw_hex":"e886020302","size":5,"soft_fail":false,"target_va":5417057840,"va":5383306149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":7412,"parent_record_offset":6916,"ancestor_record_offset":6916,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":7048,"parent_record_offset":6916,"ancestor_record_offset":6916,"inlinee":49142,"source_line":60,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::circle::Circle2d::new_rotated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":763,"record_offset":7580,"parent_record_offset":6916,"ancestor_record_offset":6916,"inlinee":49145,"source_line":40,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":6916,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":763,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":6916,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":6916,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":763,"record_offset":7520,"parent_record_offset":7412,"ancestor_record_offset":6916,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":7076,"parent_record_offset":7048,"ancestor_record_offset":6916,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":7320,"parent_record_offset":7048,"ancestor_record_offset":6916,"inlinee":49143,"source_line":22,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":6916,"inlinee":49144,"source_line":32,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7192,"parent_record_offset":7048,"ancestor_record_offset":6916,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7256,"parent_record_offset":7192,"ancestor_record_offset":6916,"inlinee":48932,"source_line":149,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":6916,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::subtract
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":6916,"inlinee":34553,"source_line":161,"archive":"_pdb/C/modules/00763.jsonl.gz"}
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
// 完整结构及原始类型记录: circle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::circle::Circle2d
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 28; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<utils::geometry::circle::Circle2d>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["utils::geometry::circle::Circle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::circle::Circle2d>", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["utils::geometry::circle::Circle2d"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 38; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::aabb
// 信息 ID: 97820d0d48f2c40d3572345d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x8388a2 -> 0xdebae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_circle::closure_env$0> @ 0xb3d62d -> 0xdebae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2> @ 0xb3dbc7 -> 0xdebae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<forest::manage_tree_prefabs::manage_tree_prefabs::closure_env$0> @ 0x286d00d -> 0xdebae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$2> @ 0x2aff196 -> 0xdebae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0> @ 0x2aff5a9 -> 0xdebae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::convert_preplaced_trees::convert_preplaced_trees::closure_env$0> @ 0x2aff9fd -> 0xdebae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::area
// 信息 ID: bd9e717bacb4fcfb871d5d0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio @ 0x92bef3 -> 0xdebb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_group_move_audio::wall_group_move_audio @ 0x92ba23 -> 0xdebb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0xab5113 -> 0xdebb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::set_parameters @ 0xab69d2 -> 0xdebb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::get_building_size_01 @ 0xab6c31 -> 0xdebb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle @ 0xb7ffb1 -> 0xdebb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle @ 0xb80264 -> 0xdebb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle @ 0xb800fb -> 0xdebb10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve2
// 信息 ID: 5b923217285ee48e415b4168
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2 @ 0xa62480 -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5c0fd -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb62a51 -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7e0a9 -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1 @ 0xbdbab4 -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::calc_shape_height::calc_shape_height_and_stone_foundation @ 0xbec282 -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x14838cd -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x148d7d5 -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x148e8db -> 0xdebd10 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x23b0f56 -> 0xdebd10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve2_range
// 信息 ID: 51af43c908c95a597d49e770
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xc0ba22 -> 0xdeb4c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_curve3
// 信息 ID: 494dd86951777dc4bd503d5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3 @ 0xa624c6 -> 0xdebe30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac3415 -> 0xdebe30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line @ 0xaf1b07 -> 0xdebe30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7ca58 -> 0xdebe30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure$0 | core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier::closure$0::closure_env$0,tuple$<> > | country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode::closure$1::closure$0 | core::ops::function::FnOnce::call_once<country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode::closure$1::closure_env$0,tuple$<> > @ 0xb7f15a -> 0xdebe30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0::closure$0 | core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0::closure_env$0,tuple$<> > @ 0xb7f688 -> 0xdebe30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1::closure$0 @ 0xbe613b -> 0xdebe30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_terrain_focus_mode::ui_terrain_focus_mode @ 0x1abc9d1 -> 0xdebe30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x28d0c48 -> 0xdebe30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points2
// 信息 ID: 455088c8c50180f62a3df92b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fdb34 -> 0xdeb1b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0xa3e70c -> 0xdeb1b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xb44388 -> 0xdeb1b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points3; module=763, inline_record=1024; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2; module=763, inline_record=7748; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=763, inline_record=8208; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points2_ptcount
// 信息 ID: 9eab739339818c980fd32ea7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x2833615 -> 0xdeb870 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x283389c -> 0xdeb870 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x2833d48 -> 0xdeb870 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points2; module=763, inline_record=800; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::as_points3
// 信息 ID: b9f9382b6e85129527a3461f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<false> @ 0x36d669 -> 0xdeb240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::playermade_stairs_assemble_visuals::playermade_stairs_assemble<true> @ 0x37fb8e -> 0xdeb240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860e6d -> 0xdeb240 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fdafd -> 0xdeb240 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0xac9d58 -> 0xdeb240 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=763, inline_record=8184; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::closest_point_on_perimeter
// 信息 ID: aba6f169f80ee5d340664dcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::check_if_dormer_ridge_is_within_roof_bounds @ 0x8d7567 -> 0xdeba60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7d676 -> 0xdeba60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x14e5861 -> 0xdeba60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests::closure$0 @ 0x28c825e -> 0xdeba60 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests::closure$0 @ 0x28c83d9 -> 0xdeba60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::contains_point
// 信息 ID: c6ef034051033bce8d015dd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 29 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x909e4a -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x909d69 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_draw_wall::ui_draw_wall @ 0x9099d2 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::decoclutter_show_pins::decoclutter_show_pins @ 0x929170 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0xa5bf6a -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_pruner::ivy_pruner @ 0xa5bf1a -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62187 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa621aa -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa621cd -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa621f6 -> 0xdeb490 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa622dd -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62303 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62325 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62347 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62369 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::slice::iter::impl$171::all<country_core::resources::walls::inner_wall_state::WallState,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3> @ 0xb193e8 -> 0xdeb490 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_closest_u_from_pos
// 信息 ID: da499e3bf0a01a8713fd5a3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141b8a3 -> 0xdeb890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141b8dc -> 0xdeb890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e8ca -> 0xdeb890 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e8eb -> 0xdeb890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_normal_at_u
// 信息 ID: a8ba07b293aa570ab13e8533
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::stone_foundation_surface_systems::maintain_stone_foundation_surfaces @ 0x83c0ae -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141f42a -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143fe85 -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x1464250 -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x1482716 -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x1482832 -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dc14c -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout @ 0x24dc2d7 -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x2830037 -> 0xdeb620 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_pos_at_u; module=763, inline_record=1492; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_tangent_at_u; module=763, inline_record=3720; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> >; module=2186, inline_record=7256; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; module=2186, inline_record=11716; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; module=2186, inline_record=13832; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_pos_at_u
// 信息 ID: cb6c7b00b53b0a8cb4c5af9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 45 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> > @ 0x303833 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62201 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62227 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa6224d -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62273 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62299 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa622bf -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa622cf -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa622f5 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62317 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa62339 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa6235b -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_shape @ 0xa6237d -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::is_shape_inside @ 0xb44467 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xc0b4a5 -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xc0b9dc -> 0xdeb300 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> >; module=2186, inline_record=7228; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; module=2186, inline_record=11688; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; module=2186, inline_record=13804; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::get_tangent_at_u
// 信息 ID: 39e4f9836b50d3ec48433c9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::systems::nani_update_stone_floor @ 0x25b3163 -> 0xdeb700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::intersect_circle
// 信息 ID: 3d80a2895d56d34f9f9973fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::trap_doors::generate_trap_doors @ 0x87a660 -> 0xdeb790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc569 -> 0xdeb790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cd670 -> 0xdeb790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::intersect_ray
// 信息 ID: b164fa906b7a3e846447bd69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5bd24 -> 0xdeb3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_shapes::ui_move_shapes @ 0xb5be1a -> 0xdeb3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0xb8aec6 -> 0xdeb3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::utils_intersect::line_x_shape_overlap @ 0xbe6bee -> 0xdeb3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env$4> @ 0x10cc46b -> 0xdeb3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env$4> @ 0x10cd450 -> 0xdeb3c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::intersects_circle; module=764, inline_record=24496; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::is_point_inside
// 信息 ID: f5755fcfdf3945fe7bd963bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize @ 0x1481a45 -> 0xdeb6b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::lerp
// 信息 ID: d56872e2f1361bbefd358fd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xb447a7 -> 0xdebb30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade @ 0xb4484c -> 0xdebb30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::new
// 信息 ID: 519ca9aeebbd498a3abec529
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7d9da2e094c5e3aaa2b1ca08: void (*)(bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 26 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x860e5a -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug @ 0xac33c4 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0xac9d3e -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0xb8a7fa -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::garden::draw_plant_paint::draw_plant_paint @ 0xbc6334 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xbfea81 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xc0b495 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xc0b9d1 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::draw_path::draw_path @ 0xc1d7c7 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::draw_path::draw_path @ 0xc1d910 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::private::de::content::ContentRefDeserializer<serde_json::error::Error> > @ 0xc86379 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> > @ 0xc8675f -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::garden::draw_garden::draw_garden @ 0x17df78a -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain @ 0x1820577 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::commands::Commands)> > @ 0x193ed4e -> 0xdebac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::system::commands::Commands)> > @ 0x196b453 -> 0xdebac0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::new_rotated
// 信息 ID: 3397ee66df8d053b0344cbdd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::trap_doors::generate_trap_doors @ 0x87a648 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7d0dc -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7ca1a -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7cbe5 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle @ 0xb7df44 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y @ 0xbe8c29 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x148378f -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2824eef -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2822896 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x28229d1 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2823221 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2823243 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282fcbc -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x2935ee9 -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::move_critters @ 0x293655e -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::shared::critters_avoid_collisions @ 0x29372fb -> 0xdeb2e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=763, inline_record=7580; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::perimeter
// 信息 ID: 156de1a0ae2e67258dd69326
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier::closure$0 @ 0xb7f4ac -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::structure_outline::structure_outline::closure$1 @ 0xbdbb5e -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143fe44 -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0x1481ba2 -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x14823f5 -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0x14838d6 -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0x148e8a6 -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282fcff -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_circle::construct_timberframe_circle @ 0x28335a3 -> 0xdebf70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points2; module=763, inline_record=692; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_points3; module=763, inline_record=1052; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2_range; module=763, inline_record=2436; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve2; module=763, inline_record=7776; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::as_curve3; module=763, inline_record=8236; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::signed_distance
// 信息 ID: 9e05e4288b75ab9adee587d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars @ 0x1482911 -> 0xdeb6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x14d6a7e -> 0xdeb6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door @ 0x14d6aa1 -> 0xdeb6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::with_expanded_diameter
// 信息 ID: 5924b66deb658a00c5d218aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x2861d01 -> 0xdeba30 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::hide_trees::hide_trees @ 0x2862248 -> 0xdeba30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::Circle2d::yaw_radians
// 信息 ID: ea3b854f8688b5575b9f9c30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter; module=570, inline_record=4208; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians; module=584, inline_record=21384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize
// 信息 ID: c08a6bfe0770e8a9e8c5f941
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::valid_enclosure::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<serde_json::error::Error> >; module=609, inline_record=54996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: e9b1cb3ec831fe6201a8606d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<utils::geometry::circle::_::impl$0::deserialize::__FieldVisitor> @ 0x1af46c0 -> 0xdeb190 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>
// 信息 ID: b23dfae130e4d99e3ff31ed8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 79db93d4c2bf3ad9e26cad37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=27936; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=28048; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=28160; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=28824; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=29032; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=29240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 73439b661bbdc65c766daa31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=27868; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=28516; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=28972; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=29004; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=29180; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=29212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 98df8697fe9938cf3a6d27af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=27628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 39359af173a82a1fc3d72ab6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<utils::geometry::circle::_::impl$0::deserialize::__Visitor> @ 0x1af46f0 -> 0xdeb170 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 89ed1465f3d5405968230a2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=27196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 87809c2f1653741d9fa965a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=31072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 448c22e66d3191149facda66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] utils::geometry::circle::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x5a3840 -> 0x248100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::valid_enclosure::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xc86070 -> 0x1af5370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 693b7818b5a61eaa06533cd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::geometry::circle::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<utils::geometry::circle::Circle2d> @ 0x1b801eb -> 0x1af5220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$0::as_curve2_range::closure$0
// 信息 ID: 4429e05379c96b190ae66aac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::circle::impl$0::as_curve2_range", "captures": "unknown"}
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_curve2_range::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_curve2_range::closure_env$0> >; module=2186, inline_record=6932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$0::as_points2_ptcount::closure$0
// 信息 ID: 42691060e0ab0f740b284407
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::geometry::circle::impl$0::as_points2_ptcount", "captures": "unknown"}
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; module=81, inline_record=198620; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::glade_border::GladeBorder::is_shape_inside; module=567, inline_record=77644; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; module=1545, inline_record=50808; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; module=1643, inline_record=563536; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; module=1643, inline_record=687996; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >; module=2186, inline_record=11508; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0> >; module=2186, inline_record=13624; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3799, inline_record=2384; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$4::fmt
// 信息 ID: 19d36cee8bf85316756dba3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<utils::geometry::circle::Circle2d>; module=1774, inline_record=72684; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$80::fmt<utils::geometry::circle::Circle2d>; module=4988, inline_record=412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::impl$7::eq
// 信息 ID: 63411ff77fe1d1eb432c130d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::ui_move_wall_group; module=561, inline_record=32296; 不是独立调用边。
// [PDB 内联宿主] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners; module=2741, inline_record=13952; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3779, inline_record=4680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::circle::is_default
// 信息 ID: 9bd5963a5d5585c5dabafaaa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=76, inline_record=26112; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1774, inline_record=82332; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1774, inline_record=82728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
