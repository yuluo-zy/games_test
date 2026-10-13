// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/circular_f32.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 56157488c9f61057e590ed78f955b304d3fee6b04d90741291398290ba7a0ac0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circular_f32.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 76, 126, 540, 570, 763, 1636, 1720, 1774, 1795, 1802, 1807, 2186, 3631, 3779, 5063
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
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
// 函数符号: utils::circular_f32::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8266
// 首条位置: {"module":1795,"inlinee":33382}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$0::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1807,"record_offset":400,"rva":29363424,"va":5398072544}
// 编译实例 RVAs: 0x1c00ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-13248485","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5398072544,"raw_hex":"e91bd835ff","size":5,"soft_fail":false,"target_va":5384824080,"va":5398072560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8252
// 首条位置: {"module":1795,"inlinee":33362}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8253
// 首条位置: {"module":1795,"inlinee":33363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":22776,"parent_record_offset":22748,"ancestor_record_offset":22748,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":22804,"parent_record_offset":22776,"ancestor_record_offset":22748,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":22832,"parent_record_offset":22804,"ancestor_record_offset":22748,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":22860,"parent_record_offset":22832,"ancestor_record_offset":22748,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81F0
// 首条位置: {"module":540,"inlinee":33264}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15602
// 首条位置: {"module":1795,"inlinee":87554}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":22604,"parent_record_offset":22560,"ancestor_record_offset":22560,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":23228,"parent_record_offset":23204,"ancestor_record_offset":22560,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":23276,"parent_record_offset":23252,"ancestor_record_offset":22560,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":23252,"parent_record_offset":23228,"ancestor_record_offset":22560,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":23156,"parent_record_offset":23132,"ancestor_record_offset":22560,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":23132,"parent_record_offset":22604,"ancestor_record_offset":22560,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":23204,"parent_record_offset":23180,"ancestor_record_offset":22560,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":23180,"parent_record_offset":23156,"ancestor_record_offset":22560,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":22804,"parent_record_offset":22776,"ancestor_record_offset":22560,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":22832,"parent_record_offset":22804,"ancestor_record_offset":22560,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":22860,"parent_record_offset":22832,"ancestor_record_offset":22560,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":22776,"parent_record_offset":22748,"ancestor_record_offset":22560,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22908,"parent_record_offset":22604,"ancestor_record_offset":22560,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22720,"parent_record_offset":22604,"ancestor_record_offset":22560,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22664,"parent_record_offset":22604,"ancestor_record_offset":22560,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":22932,"parent_record_offset":22908,"ancestor_record_offset":22560,"inlinee":33362,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":22748,"parent_record_offset":22720,"ancestor_record_offset":22560,"inlinee":33363,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":22688,"parent_record_offset":22664,"ancestor_record_offset":22560,"inlinee":33264,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$2::expecting
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1807,"record_offset":260,"rva":29363392,"va":5398072512}
// 编译实例 RVAs: 0x1c00cc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-13248453","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5398072512,"raw_hex":"e93bd835ff","size":5,"soft_fail":false,"target_va":5384824080,"va":5398072528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15600
// 首条位置: {"module":1795,"inlinee":87552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1795,"record_offset":23868,"parent_record_offset":22220,"ancestor_record_offset":22220,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":23900,"parent_record_offset":22220,"ancestor_record_offset":22220,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":24040,"parent_record_offset":22220,"ancestor_record_offset":22220,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22272,"parent_record_offset":22220,"ancestor_record_offset":22220,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":23352,"parent_record_offset":22220,"ancestor_record_offset":22220,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":23228,"parent_record_offset":23204,"ancestor_record_offset":22220,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":23276,"parent_record_offset":23252,"ancestor_record_offset":22220,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1795,"record_offset":23252,"parent_record_offset":23228,"ancestor_record_offset":22220,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":23156,"parent_record_offset":23132,"ancestor_record_offset":22220,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1795,"record_offset":23132,"parent_record_offset":22604,"ancestor_record_offset":22220,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":23204,"parent_record_offset":23180,"ancestor_record_offset":22220,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1795,"record_offset":23180,"parent_record_offset":23156,"ancestor_record_offset":22220,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1795,"record_offset":22804,"parent_record_offset":22776,"ancestor_record_offset":22220,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1795,"record_offset":23720,"parent_record_offset":23608,"ancestor_record_offset":22220,"inlinee":7711,"source_line":1627,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":23660,"parent_record_offset":23636,"ancestor_record_offset":22220,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":23636,"parent_record_offset":23608,"ancestor_record_offset":22220,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":23684,"parent_record_offset":23660,"ancestor_record_offset":22220,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":22476,"parent_record_offset":22448,"ancestor_record_offset":22220,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":22832,"parent_record_offset":22804,"ancestor_record_offset":22220,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1795,"record_offset":22860,"parent_record_offset":22832,"ancestor_record_offset":22220,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":22448,"parent_record_offset":22420,"ancestor_record_offset":22220,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1795,"record_offset":22776,"parent_record_offset":22748,"ancestor_record_offset":22220,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":23544,"parent_record_offset":23512,"ancestor_record_offset":22220,"inlinee":31975,"source_line":2023,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":22420,"parent_record_offset":22368,"ancestor_record_offset":22220,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":23512,"parent_record_offset":23480,"ancestor_record_offset":22220,"inlinee":31974,"source_line":2178,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":22604,"parent_record_offset":22560,"ancestor_record_offset":22220,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":22368,"parent_record_offset":22320,"ancestor_record_offset":22220,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":23416,"parent_record_offset":23384,"ancestor_record_offset":22220,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":23972,"parent_record_offset":23948,"ancestor_record_offset":22220,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1795,"record_offset":23948,"parent_record_offset":23924,"ancestor_record_offset":22220,"inlinee":7793,"source_line":125,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":23996,"parent_record_offset":23972,"ancestor_record_offset":22220,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22320,"parent_record_offset":22272,"ancestor_record_offset":22220,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":23384,"parent_record_offset":23352,"ancestor_record_offset":22220,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22908,"parent_record_offset":22604,"ancestor_record_offset":22220,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22720,"parent_record_offset":22604,"ancestor_record_offset":22220,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22664,"parent_record_offset":22604,"ancestor_record_offset":22220,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":22516,"parent_record_offset":22368,"ancestor_record_offset":22220,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":23480,"parent_record_offset":23448,"ancestor_record_offset":22220,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":23608,"parent_record_offset":23544,"ancestor_record_offset":22220,"inlinee":7710,"source_line":225,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":23812,"parent_record_offset":23544,"ancestor_record_offset":22220,"inlinee":17567,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":23784,"parent_record_offset":23544,"ancestor_record_offset":22220,"inlinee":17565,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":23752,"parent_record_offset":23544,"ancestor_record_offset":22220,"inlinee":7709,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":22932,"parent_record_offset":22908,"ancestor_record_offset":22220,"inlinee":33362,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":22748,"parent_record_offset":22720,"ancestor_record_offset":22220,"inlinee":33363,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":22688,"parent_record_offset":22664,"ancestor_record_offset":22220,"inlinee":33264,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1795,"record_offset":22560,"parent_record_offset":22516,"ancestor_record_offset":22220,"inlinee":87554,"source_line":7,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15603
// 首条位置: {"module":1795,"inlinee":87555}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":24244,"parent_record_offset":24208,"ancestor_record_offset":24208,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::copysign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1795,"record_offset":24740,"parent_record_offset":24628,"ancestor_record_offset":24208,"inlinee":7711,"source_line":1627,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":24680,"parent_record_offset":24656,"ancestor_record_offset":24208,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":24656,"parent_record_offset":24628,"ancestor_record_offset":24208,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":1795,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":24208,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1795,"record_offset":24388,"parent_record_offset":24364,"ancestor_record_offset":24208,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1795,"record_offset":24364,"parent_record_offset":24324,"ancestor_record_offset":24208,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::ContentRefDeserializer::deserialize_float
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":24528,"parent_record_offset":24492,"ancestor_record_offset":24208,"inlinee":31975,"source_line":2023,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":24492,"parent_record_offset":24456,"ancestor_record_offset":24208,"inlinee":31974,"source_line":2178,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1795,"record_offset":24324,"parent_record_offset":24284,"ancestor_record_offset":24208,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":24284,"parent_record_offset":24244,"ancestor_record_offset":24208,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1795,"record_offset":24420,"parent_record_offset":24324,"ancestor_record_offset":24208,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":24456,"parent_record_offset":24420,"ancestor_record_offset":24208,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":24628,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":7710,"source_line":225,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":24888,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":17567,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":24832,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":17565,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_i8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":24600,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":17563,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":24860,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":17566,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":24772,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":17564,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize::impl$0::visit_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1795,"record_offset":24800,"parent_record_offset":24528,"ancestor_record_offset":24208,"inlinee":7709,"source_line":191,"archive":"_pdb/C/modules/01795.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33B0
// 首条位置: {"module":1636,"inlinee":13232}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":126,"record_offset":10500,"rva":4193696,"va":5372902816}
// 编译实例 RVAs: 0x3ffda0, 0xa74650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t412789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372902816,"raw_hex":"e8754c0600","size":5,"soft_fail":false,"target_va":5373315680,"va":5372902886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14964965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379671632,"raw_hex":"e8e558e400","size":5,"soft_fail":false,"target_va":5394636672,"va":5379671702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t43450055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372902816,"raw_hex":"e8c7fe9602","size":5,"soft_fail":false,"target_va":5416353040,"va":5372902980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":10764,"parent_record_offset":10500,"ancestor_record_offset":10500,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":11304,"parent_record_offset":10500,"ancestor_record_offset":10500,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":11276,"parent_record_offset":10500,"ancestor_record_offset":10500,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":11128,"parent_record_offset":11096,"ancestor_record_offset":10500,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":11000,"parent_record_offset":10976,"ancestor_record_offset":10500,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":11056,"parent_record_offset":11028,"ancestor_record_offset":10500,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":10500,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":10500,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":126,"record_offset":11028,"parent_record_offset":11000,"ancestor_record_offset":10500,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":10940,"parent_record_offset":10916,"ancestor_record_offset":10500,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":11096,"parent_record_offset":10940,"ancestor_record_offset":10500,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":10892,"parent_record_offset":10868,"ancestor_record_offset":10500,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":11216,"parent_record_offset":10940,"ancestor_record_offset":10500,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":126,"record_offset":10976,"parent_record_offset":10940,"ancestor_record_offset":10500,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":126,"record_offset":10916,"parent_record_offset":10892,"ancestor_record_offset":10500,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":10500,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":11364,"parent_record_offset":11336,"ancestor_record_offset":10500,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":10792,"parent_record_offset":10764,"ancestor_record_offset":10500,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":126,"record_offset":11336,"parent_record_offset":11304,"ancestor_record_offset":10500,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":126,"record_offset":10868,"parent_record_offset":10844,"ancestor_record_offset":10500,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":126,"record_offset":10844,"parent_record_offset":10816,"ancestor_record_offset":10500,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00126.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::impl$4::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8220
// 首条位置: {"module":540,"inlinee":33312}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2968
// 首条位置: {"module":76,"inlinee":10600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::set_value
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1807,"record_offset":1892,"rva":29364192,"va":5398073312}
// 编译实例 RVAs: 0x1c00fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18984496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398073312,"raw_hex":"e830ae2101","size":5,"soft_fail":false,"target_va":5417057840,"va":5398073339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1807,"record_offset":2020,"parent_record_offset":1892,"ancestor_record_offset":1892,"inlinee":49144,"source_line":32,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1807,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":1892,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF7
// 首条位置: {"module":763,"inlinee":49143}
// 编译实例 RVAs: 0x1c00d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18985159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398072656,"raw_hex":"e8c7b02101","size":5,"soft_fail":false,"target_va":5417057840,"va":5398072676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":7320,"inlinee":49144,"source_line":32,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":7320,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::project_unbounded
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF8
// 首条位置: {"module":763,"inlinee":49144}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":7344,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::as_f32
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=18
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF4
// 首条位置: {"module":763,"inlinee":49140}
// 编译实例 RVAs: 0x1c00ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18984787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398073024,"raw_hex":"e853af2101","size":5,"soft_fail":false,"target_va":5417057840,"va":5398073048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1524,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::distance
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1807,"record_offset":1528,"rva":29363984,"va":5398073104}
// 编译实例 RVAs: 0x1c00f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18984687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398073104,"raw_hex":"e8efae2101","size":5,"soft_fail":false,"target_va":5417057840,"va":5398073148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1807,"record_offset":1660,"parent_record_offset":1528,"ancestor_record_offset":1528,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1807,"record_offset":1772,"parent_record_offset":1528,"ancestor_record_offset":1528,"inlinee":48861,"source_line":127,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1807,"record_offset":1800,"parent_record_offset":1772,"ancestor_record_offset":1528,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1807,"record_offset":1856,"parent_record_offset":1772,"ancestor_record_offset":1528,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1807,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":1528,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::add
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1807,"record_offset":540,"rva":29363456,"va":5398072576}
// 编译实例 RVAs: 0x1c00d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18985235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398072576,"raw_hex":"e813b12101","size":5,"soft_fail":false,"target_va":5417057840,"va":5398072600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1807,"record_offset":664,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":49144,"source_line":32,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1807,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":540,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01807.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::circular_f32::CircularF32::lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xBFF6
// 首条位置: {"module":763,"inlinee":49142}
// 编译实例 RVAs: 0x1c00d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: circular_f32.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18985061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398072720,"raw_hex":"e865b02101","size":5,"soft_fail":false,"target_va":5417057840,"va":5398072774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::as_f32
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":7076,"parent_record_offset":7048,"ancestor_record_offset":7048,"inlinee":49140,"source_line":36,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::circular_f32::CircularF32::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":7320,"parent_record_offset":7048,"ancestor_record_offset":7048,"inlinee":49143,"source_line":22,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7192,"parent_record_offset":7048,"ancestor_record_offset":7048,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::CircularF32::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":763,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":7048,"inlinee":49144,"source_line":32,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7256,"parent_record_offset":7192,"ancestor_record_offset":7048,"inlinee":48932,"source_line":149,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":7048,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00763.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::subtract
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":763,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":7048,"inlinee":34553,"source_line":161,"archive":"_pdb/C/modules/00763.jsonl.gz"}
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
// 完整结构及原始类型记录: circular_f32.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::circular_f32::CircularF32
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,utils::circular_f32::CircularF32>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "utils::circular_f32::CircularF32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<utils::circular_f32::CircularF32>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["utils::circular_f32::CircularF32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<utils::circular_f32::CircularF32>", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["utils::circular_f32::CircularF32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circular_f32.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circular_f32.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::DeletedAutoClutter>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>"]
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::add
// 信息 ID: 0c22ca3042bb620749048d94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_wall_group::WallGroupMember::target_state @ 0xb11c0e -> 0x1c00d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::as_f32
// 信息 ID: 7ff70125060f9958c0856440
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 23 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter @ 0xb580bd -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb622c8 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb62641 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians @ 0xbe85f2 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143f746 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0x143f3e3 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x14591c2 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x14591d2 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x1457a96 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x1457aa6 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145bb68 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::construct_wooden_wall_boards<system_wall_constructor::construct_walls::construct_walls::closure_env$9,system_wall_constructor::construct_walls::construct_walls::closure_env$10,system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145bb79 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::emit_curved_horizontal_wall_board_piece<system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145f879 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::construct_wall_boards::emit_curved_horizontal_wall_board_piece<system_wall_constructor::construct_walls::construct_walls::closure_env$11> @ 0x145f885 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders @ 0x147bc39 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_wall_transform_gizmos::wall_rotation_gizmo @ 0x1abaac6 -> 0x1c00ec0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_pos_at_u; module=763, inline_record=1524; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_normal_at_u; module=763, inline_record=2848; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_tangent_at_u; module=763, inline_record=3748; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_closest_u_from_pos; module=763, inline_record=5376; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=763, inline_record=7076; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=763, inline_record=7132; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::lerp; module=1807, inline_record=1040; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::lerp; module=1807, inline_record=1096; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::distance
// 信息 ID: c0a3964f1c9a5345b5d9d934
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb626af -> 0x1c00f10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb629ac -> 0x1c00f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::lerp
// 信息 ID: e3c3c73d11230f3a4ab90c4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb625f3 -> 0x1c00d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_wall_transform_gizmos::wall_rotation_gizmo @ 0x1aba9f9 -> 0x1c00d90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=763, inline_record=7048; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::new
// 信息 ID: cc2bc21dc69abc4e4c741f1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb625a2 -> 0x1c00d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb6295e -> 0x1c00d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb62cd9 -> 0x1c00d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_wall_transform_gizmos::wall_rotation_gizmo @ 0x1aba835 -> 0x1c00d50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=763, inline_record=7320; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::lerp; module=1807, inline_record=1280; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::project_unbounded
// 信息 ID: 8a6c16587baa4b92596e21ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::lerp; module=763, inline_record=7344; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::add; module=1807, inline_record=664; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::new; module=1807, inline_record=848; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::lerp; module=1807, inline_record=1304; 不是独立调用边。
// [PDB 内联宿主] utils::circular_f32::CircularF32::set_value; module=1807, inline_record=2020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::CircularF32::set_value
// 信息 ID: 8b34826b0ed24e8d616e347a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle @ 0xb62405 -> 0x1c00fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize
// 信息 ID: 84bc5e5cf5b2bd61782628b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=30268; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::geometry::circle::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=32756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 2dd6e0e79ec24d0acb5d5daf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<utils::circular_f32::_::impl$0::deserialize::__FieldVisitor> @ 0xa744a0 -> 0x1c00ce0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes
// 信息 ID: e218eb35f93fa01a5f062083
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=22932; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=22964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 3744b8a916b628475abb8ccd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=22748; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=23020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: e4eda377106b1b5f588965be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=22688; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=22992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 8d18c4b6d42743d0d0eeba14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=22560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 11befda1ce2cb8d26836d764
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impl$1::fmt<utils::circular_f32::_::impl$0::deserialize::__Visitor> @ 0xa74450 -> 0x1c00cc0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: c09b2d69ca3c137ec1e0801c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=22220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 680a1519ac624aac58d6f91a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<serde_json::error::Error,utils::circular_f32::_::impl$0::deserialize::__Visitor>; module=1795, inline_record=24208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::serialize
// 信息 ID: 7f9f5262065b78d8aa3707df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<utils::circular_f32::CircularF32>; module=1802, inline_record=107448; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::ser::impl$8::serialize_field<utils::circular_f32::CircularF32>; module=1802, inline_record=108464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 41e2d6ae27113ac61838b7e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] utils::circular_f32::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,utils::circular_f32::CircularF32> @ 0x467482 -> 0x3ffda0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,utils::circular_f32::CircularF32> @ 0x18b5662 -> 0xa74650 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::impl$4::fmt
// 信息 ID: a01ef9b5c35d0542e9ff6723
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<utils::circular_f32::CircularF32>; module=540, inline_record=46500; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$80::fmt<utils::circular_f32::CircularF32>; module=5063, inline_record=412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::circular_f32::impl$7::eq
// 信息 ID: 66ac9e17330b29a2d5e943c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=76, inline_record=26168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle; module=570, inline_record=94532; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=130924; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=1774, inline_record=82388; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::circle::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1774, inline_record=82784; 不是独立调用边。
// [PDB 内联宿主] system_roof::update_roof_spatial::update_roof_spatial; module=3779, inline_record=4740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
