// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/ui_settings.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 046752539f8564da3a2ffefff7fccfce20f64158c049a42226bf5acda0cfdfe6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_settings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 64, 137, 144, 153, 202, 586, 594, 1645, 1685, 1694, 1812
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x277C
// 首条位置: {"module":64,"inlinee":10108}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::missing_field::impl$0::deserialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":497952,"parent_record_offset":497928,"ancestor_record_offset":497928,"inlinee":7758,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":497976,"parent_record_offset":497952,"ancestor_record_offset":497928,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":498000,"parent_record_offset":497976,"ancestor_record_offset":497928,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":153,"record_offset":15344,"rva":4847088,"va":5373556208}
// 编译实例 RVAs: 0x49f5f0, 0x49f780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42998993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373556608,"raw_hex":"e8d11c9002","size":5,"soft_fail":false,"target_va":5416555792,"va":5373556794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":153,"record_offset":15536,"parent_record_offset":15344,"ancestor_record_offset":15344,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":153,"record_offset":15560,"parent_record_offset":15536,"ancestor_record_offset":15344,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":153,"record_offset":15584,"parent_record_offset":15560,"ancestor_record_offset":15344,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":153,"record_offset":15612,"parent_record_offset":15584,"ancestor_record_offset":15344,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1685,"record_offset":25816,"rva":25292144,"va":5394001264}
// 编译实例 RVAs: 0x181ed70, 0x181f050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22384291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394002000,"raw_hex":"e8a38e5501","size":5,"soft_fail":false,"target_va":5416386464,"va":5394002168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1685,"record_offset":26008,"parent_record_offset":25816,"ancestor_record_offset":25816,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1685,"record_offset":26032,"parent_record_offset":26008,"ancestor_record_offset":25816,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1685,"record_offset":26056,"parent_record_offset":26032,"ancestor_record_offset":25816,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1685,"record_offset":26084,"parent_record_offset":26056,"ancestor_record_offset":25816,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x26D3
// 首条位置: {"module":64,"inlinee":9939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":64,"record_offset":23532,"parent_record_offset":23508,"ancestor_record_offset":23508,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":148564,"parent_record_offset":148544,"ancestor_record_offset":148544,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":183552,"parent_record_offset":183520,"ancestor_record_offset":183520,"inlinee":7716,"source_line":1903,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":148708,"parent_record_offset":148588,"ancestor_record_offset":148544,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":23568,"parent_record_offset":23532,"ancestor_record_offset":23508,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":23632,"parent_record_offset":23532,"ancestor_record_offset":23508,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":64,"record_offset":23604,"parent_record_offset":23532,"ancestor_record_offset":23508,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":148624,"parent_record_offset":148588,"ancestor_record_offset":148544,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::fix_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":183992,"parent_record_offset":183584,"ancestor_record_offset":183520,"inlinee":7591,"source_line":441,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":183652,"parent_record_offset":183584,"ancestor_record_offset":183520,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":183692,"parent_record_offset":183652,"ancestor_record_offset":183520,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":148588,"parent_record_offset":148564,"ancestor_record_offset":148544,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":183584,"parent_record_offset":183552,"ancestor_record_offset":183520,"inlinee":7717,"source_line":1524,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":148648,"parent_record_offset":148624,"ancestor_record_offset":148544,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":183720,"parent_record_offset":183692,"ancestor_record_offset":183520,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":148672,"parent_record_offset":148648,"ancestor_record_offset":148544,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":183748,"parent_record_offset":183720,"ancestor_record_offset":183520,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9253
// 首条位置: {"module":1685,"inlinee":37459}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x277F
// 首条位置: {"module":64,"inlinee":10111}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":64,"record_offset":185740,"parent_record_offset":184736,"ancestor_record_offset":184736,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":185676,"parent_record_offset":184736,"ancestor_record_offset":184736,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1645,"record_offset":254620,"parent_record_offset":250052,"ancestor_record_offset":250052,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":254660,"parent_record_offset":250052,"ancestor_record_offset":250052,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":64,"record_offset":184816,"parent_record_offset":184736,"ancestor_record_offset":184736,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":64,"record_offset":185648,"parent_record_offset":184736,"ancestor_record_offset":184736,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":250404,"parent_record_offset":250284,"ancestor_record_offset":250052,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":64,"record_offset":185060,"parent_record_offset":185036,"ancestor_record_offset":184736,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":64,"record_offset":185088,"parent_record_offset":185060,"ancestor_record_offset":184736,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":64,"record_offset":185128,"parent_record_offset":184860,"ancestor_record_offset":184736,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":64,"record_offset":185368,"parent_record_offset":184860,"ancestor_record_offset":184736,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":185596,"parent_record_offset":184860,"ancestor_record_offset":184736,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":184996,"parent_record_offset":184860,"ancestor_record_offset":184736,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":64,"record_offset":186356,"parent_record_offset":186332,"ancestor_record_offset":184736,"inlinee":10108,"source_line":31,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":64,"record_offset":185212,"parent_record_offset":185188,"ancestor_record_offset":184736,"inlinee":9939,"source_line":5,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":64,"record_offset":185160,"parent_record_offset":184860,"ancestor_record_offset":184736,"inlinee":9971,"source_line":11,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":64,"record_offset":185484,"parent_record_offset":185456,"ancestor_record_offset":184736,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":64,"record_offset":184860,"parent_record_offset":184816,"ancestor_record_offset":184736,"inlinee":9956,"source_line":852,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":64,"record_offset":186052,"parent_record_offset":186012,"ancestor_record_offset":184736,"inlinee":9908,"source_line":873,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":64,"record_offset":186208,"parent_record_offset":186052,"ancestor_record_offset":184736,"inlinee":9910,"source_line":1050,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_tag_or_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":64,"record_offset":185036,"parent_record_offset":184860,"ancestor_record_offset":184736,"inlinee":9920,"source_line":1109,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":64,"record_offset":185556,"parent_record_offset":185484,"ancestor_record_offset":184736,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1645,"record_offset":252928,"parent_record_offset":252904,"ancestor_record_offset":250052,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":64,"record_offset":185188,"parent_record_offset":184860,"ancestor_record_offset":184736,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":251408,"parent_record_offset":251384,"ancestor_record_offset":250052,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":250320,"parent_record_offset":250284,"ancestor_record_offset":250052,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":250584,"parent_record_offset":250552,"ancestor_record_offset":250052,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":250616,"parent_record_offset":250584,"ancestor_record_offset":250052,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":250656,"parent_record_offset":250616,"ancestor_record_offset":250052,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":250140,"parent_record_offset":250104,"ancestor_record_offset":250052,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":250552,"parent_record_offset":250524,"ancestor_record_offset":250052,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":250284,"parent_record_offset":250260,"ancestor_record_offset":250052,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":250260,"parent_record_offset":250236,"ancestor_record_offset":250052,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":252952,"parent_record_offset":252928,"ancestor_record_offset":250052,"inlinee":7602,"source_line":1910,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":250344,"parent_record_offset":250320,"ancestor_record_offset":250052,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":250684,"parent_record_offset":250656,"ancestor_record_offset":250052,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=17
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":250368,"parent_record_offset":250344,"ancestor_record_offset":250052,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":250712,"parent_record_offset":250684,"ancestor_record_offset":250052,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E4A
// 首条位置: {"module":1645,"inlinee":81482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1645,"record_offset":255900,"parent_record_offset":255120,"ancestor_record_offset":255120,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":255168,"parent_record_offset":255120,"ancestor_record_offset":255120,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":255320,"parent_record_offset":255272,"ancestor_record_offset":255120,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":255452,"parent_record_offset":255428,"ancestor_record_offset":255120,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":255192,"parent_record_offset":255168,"ancestor_record_offset":255120,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 5, 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":153,"record_offset":248,"rva":4841696,"va":5373550816}
// 编译实例 RVAs: 0x49e0e0, 0x49e580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403d0050
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-845195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373552000,"raw_hex":"e8751af3ff","size":5,"soft_fail":false,"target_va":5372706896,"va":5373552086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403d0470
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-844102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373552000,"raw_hex":"e8ba1ef3ff","size":5,"soft_fail":false,"target_va":5372707952,"va":5373552049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ddac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-788184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373550816,"raw_hex":"e828f9f3ff","size":5,"soft_fail":false,"target_va":5372762816,"va":5373550995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403e0d20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-775340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373550816,"raw_hex":"e8542bf4ff","size":5,"soft_fail":false,"target_va":5372775712,"va":5373551047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403e6e60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-750340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373550816,"raw_hex":"e8fc8cf4ff","size":5,"soft_fail":false,"target_va":5372800608,"va":5373550943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403e8ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-742405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373550816,"raw_hex":"e8fbabf4ff","size":5,"soft_fail":false,"target_va":5372808896,"va":5373551296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403e9be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-738629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373550816,"raw_hex":"e8bbbaf4ff","size":5,"soft_fail":false,"target_va":5372812256,"va":5373550880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":153,"record_offset":760,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":153,"record_offset":468,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":153,"record_offset":700,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::impl$3::serialize_unit_variant
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/ser/mod.rs
//     evidence={"module":153,"record_offset":2344,"parent_record_offset":2132,"ancestor_record_offset":2132,"inlinee":7138,"source_line":873,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":153,"record_offset":812,"parent_record_offset":784,"ancestor_record_offset":248,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":153,"record_offset":2372,"parent_record_offset":2344,"ancestor_record_offset":2132,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/ser/mod.rs
//     evidence={"module":153,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":248,"inlinee":7141,"source_line":1117,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":153,"record_offset":940,"rva":4842400,"va":5373551520}
// 编译实例 RVAs: 0x49e3a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1723955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373551520,"raw_hex":"e8cdb1e5ff","size":5,"soft_fail":false,"target_va":5371827984,"va":5373551934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045f200
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-258641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373551520,"raw_hex":"e8af0dfcff","size":5,"soft_fail":false,"target_va":5373293056,"va":5373551692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140461d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-247546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373551520,"raw_hex":"e80639fcff","size":5,"soft_fail":false,"target_va":5373304112,"va":5373551653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-235919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373551520,"raw_hex":"e87166fcff","size":5,"soft_fail":false,"target_va":5373315680,"va":5373551594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42801059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373551520,"raw_hex":"e8a3178d02","size":5,"soft_fail":false,"target_va":5416353040,"va":5373551976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t43006113","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373551520,"raw_hex":"e9a1389002","size":5,"soft_fail":false,"target_va":5416557744,"va":5373551626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":1220,"parent_record_offset":940,"ancestor_record_offset":940,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":1984,"parent_record_offset":940,"ancestor_record_offset":940,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":1764,"parent_record_offset":940,"ancestor_record_offset":940,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":1616,"parent_record_offset":1584,"ancestor_record_offset":940,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":1484,"parent_record_offset":1456,"ancestor_record_offset":940,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":1544,"parent_record_offset":1512,"ancestor_record_offset":940,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":940,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":940,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":153,"record_offset":1512,"parent_record_offset":1484,"ancestor_record_offset":940,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":1416,"parent_record_offset":1388,"ancestor_record_offset":940,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":1584,"parent_record_offset":1416,"ancestor_record_offset":940,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":940,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":1704,"parent_record_offset":1416,"ancestor_record_offset":940,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":153,"record_offset":1456,"parent_record_offset":1416,"ancestor_record_offset":940,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":153,"record_offset":1388,"parent_record_offset":1360,"ancestor_record_offset":940,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":940,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":940,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":940,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":153,"record_offset":2012,"parent_record_offset":1984,"ancestor_record_offset":940,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":153,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":940,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":153,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":940,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1685,"record_offset":11884,"rva":25288144,"va":5393997264}
// 编译实例 RVAs: 0x181ddd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403aeac0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21427347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997264,"raw_hex":"e86d0bb9fe","size":5,"soft_fail":false,"target_va":5372570304,"va":5393997646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b4b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t617774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997264,"raw_hex":"e82e6d0900","size":5,"soft_fail":false,"target_va":5394615184,"va":5393997405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bb310
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t644231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997264,"raw_hex":"e887d40900","size":5,"soft_fail":false,"target_va":5394641680,"va":5393997444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418beb40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t658713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997264,"raw_hex":"e8190d0a00","size":5,"soft_fail":false,"target_va":5394656064,"va":5393997346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22355347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997264,"raw_hex":"e8931d5501","size":5,"soft_fail":false,"target_va":5416353040,"va":5393997688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t22560361","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393997264,"raw_hex":"e9693e5801","size":5,"soft_fail":false,"target_va":5416557744,"va":5393997378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1685,"record_offset":12160,"parent_record_offset":11884,"ancestor_record_offset":11884,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1685,"record_offset":12708,"parent_record_offset":11884,"ancestor_record_offset":11884,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1685,"record_offset":12560,"parent_record_offset":12528,"ancestor_record_offset":11884,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1685,"record_offset":12428,"parent_record_offset":12400,"ancestor_record_offset":11884,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1685,"record_offset":12488,"parent_record_offset":12456,"ancestor_record_offset":11884,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1685,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":11884,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1685,"record_offset":12584,"parent_record_offset":12560,"ancestor_record_offset":11884,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1685,"record_offset":12456,"parent_record_offset":12428,"ancestor_record_offset":11884,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1685,"record_offset":12360,"parent_record_offset":12332,"ancestor_record_offset":11884,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1685,"record_offset":12528,"parent_record_offset":12360,"ancestor_record_offset":11884,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1685,"record_offset":12304,"parent_record_offset":12276,"ancestor_record_offset":11884,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1685,"record_offset":12648,"parent_record_offset":12360,"ancestor_record_offset":11884,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1685,"record_offset":12400,"parent_record_offset":12360,"ancestor_record_offset":11884,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1685,"record_offset":12332,"parent_record_offset":12304,"ancestor_record_offset":11884,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::begin_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1685,"record_offset":12216,"parent_record_offset":12188,"ancestor_record_offset":11884,"inlinee":12492,"source_line":2018,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1685,"record_offset":12188,"parent_record_offset":12160,"ancestor_record_offset":11884,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":1685,"record_offset":12276,"parent_record_offset":12248,"ancestor_record_offset":11884,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":1685,"record_offset":12248,"parent_record_offset":12216,"ancestor_record_offset":11884,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35BA
// 首条位置: {"module":137,"inlinee":13754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$4::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":63368,"rva":12548640,"va":5381257760}
// 编译实例 RVAs: 0xbf7a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9254
// 首条位置: {"module":1685,"inlinee":37460}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2750
// 首条位置: {"module":64,"inlinee":10064}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":104100,"parent_record_offset":104048,"ancestor_record_offset":104048,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1685,"record_offset":13556,"parent_record_offset":13496,"ancestor_record_offset":13496,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":64,"record_offset":104072,"parent_record_offset":104048,"ancestor_record_offset":104048,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1685,"record_offset":12932,"rva":25288592,"va":5393997712}
// 编译实例 RVAs: 0x181df90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141655420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1870905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997712,"raw_hex":"e8c773e3ff","size":5,"soft_fail":false,"target_va":5392127008,"va":5393997908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141655fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1867999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997712,"raw_hex":"e8217fe3ff","size":5,"soft_fail":false,"target_va":5392129952,"va":5393997946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416589b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1857127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997712,"raw_hex":"e899a9e3ff","size":5,"soft_fail":false,"target_va":5392140720,"va":5393997842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68840
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22325317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997712,"raw_hex":"e845a85401","size":5,"soft_fail":false,"target_va":5416323136,"va":5393997814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t22326787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393997712,"raw_hex":"e803ae5401","size":5,"soft_fail":false,"target_va":5416324848,"va":5393998056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::de::impl$7::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1685,"record_offset":13148,"parent_record_offset":12932,"ancestor_record_offset":12932,"inlinee":14882,"source_line":1871,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1685,"record_offset":13556,"parent_record_offset":13496,"ancestor_record_offset":12932,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":1685,"record_offset":13496,"parent_record_offset":13148,"ancestor_record_offset":12932,"inlinee":10064,"source_line":31,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1685,"record_offset":13528,"parent_record_offset":13496,"ancestor_record_offset":12932,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1685,"record_offset":13408,"parent_record_offset":13256,"ancestor_record_offset":12932,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1685,"record_offset":13256,"parent_record_offset":13148,"ancestor_record_offset":12932,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1685,"record_offset":13308,"parent_record_offset":13256,"ancestor_record_offset":12932,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1685,"record_offset":13432,"parent_record_offset":13408,"ancestor_record_offset":12932,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1685,"record_offset":13336,"parent_record_offset":13308,"ancestor_record_offset":12932,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1685,"record_offset":13456,"parent_record_offset":13432,"ancestor_record_offset":12932,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1685,"record_offset":13364,"parent_record_offset":13336,"ancestor_record_offset":12932,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01685.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":47768,"rva":9754240,"va":5378463360}
// 编译实例 RVAs: 0x94d680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::previous
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":47900,"rva":10366352,"va":5379075472}
// 编译实例 RVAs: 0x9e2d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3744
// 首条位置: {"module":144,"inlinee":14148}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_unit_variant
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":183120,"parent_record_offset":183100,"ancestor_record_offset":183100,"inlinee":7408,"source_line":214,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":183144,"parent_record_offset":183120,"ancestor_record_offset":183100,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::ToolbarAlignment::is_horizontal
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":47620,"rva":12544784,"va":5381253904}
// 编译实例 RVAs: 0xbf6b10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::ui_settings::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":586,"record_offset":63500,"rva":12548672,"va":5381257792}
// 编译实例 RVAs: 0xbf7a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1876842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381257792,"raw_hex":"e86aa31c00","size":5,"soft_fail":false,"target_va":5383134720,"va":5381257873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: ui_settings.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3566183","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381257792,"raw_hex":"e9676a3600","size":5,"soft_fail":false,"target_va":5384824080,"va":5381257892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: ui_settings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::ui_settings::ToolbarAlignment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "toolbar::mei::widgets::tweaker::TweakableEnum", "arguments": ["country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> >", "constructor": "toolbar::mei::widgets::tweaker::impl$13::tweaker", "arguments": ["toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "country_core::resources::ui_settings::ToolbarAlignment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::ui_settings::UiSettings
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter,country_core::resources::ui_settings::UiSettings>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::UiSettings>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::UiSettings>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::ser::impl$9::serialize_field<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter,country_core::resources::ui_settings::UiSettings>", "constructor": "serde_json::ser::impl$9::serialize_field", "arguments": ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::UiSettings>", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >", "str$", "country_core::resources::ui_settings::UiSettings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_settings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_settings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::ToolbarAlignment::is_horizontal
// 信息 ID: b9d9621482e99493a5e38d3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_startup::setup_toolbar @ 0x2969005 -> 0xbf6b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::toolbar_startup::setup_toolbar @ 0x2969058 -> 0xbf6b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar @ 0x2b339a2 -> 0xbf6b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar @ 0x2b339f5 -> 0xbf6b10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize
// 信息 ID: 86b405444bdef81198c53106
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_config::_::impl$0::deserialize::__Visitor>; module=64, inline_record=180312; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=64, inline_record=186356; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=494444; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=497928; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::input::input_stream::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=501612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 5c020df7046a43db491c22d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor>; module=1685, inline_record=38256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 322df65b87a188ab23248449
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x218a47 -> 0x49f5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor> @ 0x57450e -> 0x49f5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor> @ 0x5745c6 -> 0x49f5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::resources::ui_settings::_::impl$0::deserialize::__FieldVisitor> @ 0x574544 -> 0x49f5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> > @ 0x1f5e46 -> 0x49f780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: d045cb7fb00974b306551a56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x165f5cb -> 0x181ed70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$13::variant_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> > @ 0x16554c7 -> 0x181f050 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$16::variant_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> > @ 0x1658a6f -> 0x181f050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 7c90c7e713d2a53ebf2b1532
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> >; module=64, inline_record=23508; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=64, inline_record=185212; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=64, inline_record=185336; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=64, inline_record=185456; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$13::variant_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> >; module=1645, inline_record=148544; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$16::variant_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::resources::ui_settings::_::impl$0::deserialize::__Field> >; module=1645, inline_record=183520; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=250236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 48dddbefa346167503512c1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1685, inline_record=37616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: a2364068df7d89ded191cc70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=64, inline_record=104048; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=1685, inline_record=13496; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >; module=1685, inline_record=13672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 2447688e2d8bac16c8b360e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=64, inline_record=184736; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=250052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: cc22230fc2ede0fae843984a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=255120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::deserialize<ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
// 信息 ID: c997d0473f6c133ca013c5a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::StrRead"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::StrRead>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::deserialize -> ["ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x165fa40 -> 0x181df90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::next
// 信息 ID: 8abd0875c37eb58a88a417e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 22 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224d4a -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x225025 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x2252da -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x2255b5 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x225e5a -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x226135 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x227a2a -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x227d05 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::systems::main_camera::CameraRotationPivot> > @ 0x22905a -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::systems::main_camera::CameraRotationPivot> > @ 0x229335 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,core::iter::adapters::filter_map::FilterMap<enum_iterator::All<country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$4::closure_env$0> > @ 0x2fe6bf -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,core::iter::adapters::filter_map::FilterMap<enum_iterator::All<country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$4::closure_env$0> > @ 0x2fe7a3 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::nani_capacity_debug_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x54b791 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::nani_capacity_debug_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x54bbf9 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::nani_capacity_debug_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x54be94 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$17::closure$1::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x58d2a0 -> 0x94d680 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::previous
// 信息 ID: 76ccc81952fa883ff5d0c082
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x227acb -> 0x9e2d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x227a47 -> 0x9e2d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::decoclutter_picker::decoclutter_picker_menu @ 0x296f6ea -> 0x9e2d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize
// 信息 ID: e3c3e76c1e1d4416f0b5bcc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>; module=144, inline_record=183100; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::ToolbarAlignment>; module=1694, inline_record=106972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: c5d635dc9da798027368acf1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::UiSettings> @ 0x3e69a9 -> 0x49e0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::resources::ui_settings::ToolbarAlignment> @ 0x3de269 -> 0x49e580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: cea6a23bf6f8deb87521aded
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,country_core::resources::ui_settings::UiSettings> @ 0x46b0b2 -> 0x49e3a0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> > >
// 信息 ID: 2aa747f9fe483e266a78475c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::PrettyFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::ui_settings::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::PrettyFormatter> >,str$,country_core::resources::ui_settings::UiSettings> @ 0x18b74b9 -> 0x181ddd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$1::fmt
// 信息 ID: 28644e9d223923f979b7031f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x227b0f -> 0xbf7a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$3::clone
// 信息 ID: 5a3b651cd63b532537243df2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_config::impl$12::clone; module=137, inline_record=220904; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=202, inline_record=43272; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::input::input_record_replay::input_record_replay; module=594, inline_record=12712; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::backend::input::input_record_replay::input_record_replay; module=594, inline_record=13608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::ui_settings::impl$4::default
// 信息 ID: fdaaf5247372424d22d78225
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::resources::ui_settings::_::impl$0::deserialize::__Visitor> @ 0x218f4f -> 0xbf7a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
