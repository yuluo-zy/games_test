// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/theme-fetcher/src/checked_zip.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4f8bc0cf2763140d7bf048e8ba1e982ca2ba1c0fd9cfca8c4fa6ffaf56171be1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: checked_zip.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 893, 905, 910, 2329
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.8"},{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.4"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use theme_fetcher; // namespace candidate
//   package identities: [{"name":"theme-fetcher","source":"local_or_path","version":"0.1.0"}]
// use zip; // namespace candidate
//   package identities: [{"name":"zip","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF98E
// 首条位置: {"module":905,"inlinee":63886}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF99C
// 首条位置: {"module":905,"inlinee":63900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":905,"record_offset":35516,"parent_record_offset":35484,"ancestor_record_offset":35484,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":905,"record_offset":35540,"parent_record_offset":35516,"ancestor_record_offset":35484,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":35564,"parent_record_offset":35540,"ancestor_record_offset":35484,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":35592,"parent_record_offset":35564,"ancestor_record_offset":35484,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF99B
// 首条位置: {"module":905,"inlinee":63899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":35320,"parent_record_offset":35292,"ancestor_record_offset":35292,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":35456,"parent_record_offset":35348,"ancestor_record_offset":35292,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":905,"record_offset":35540,"parent_record_offset":35516,"ancestor_record_offset":35292,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":35564,"parent_record_offset":35540,"ancestor_record_offset":35292,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":35592,"parent_record_offset":35564,"ancestor_record_offset":35292,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":905,"record_offset":35516,"parent_record_offset":35484,"ancestor_record_offset":35292,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35400,"parent_record_offset":35348,"ancestor_record_offset":35292,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35348,"parent_record_offset":35320,"ancestor_record_offset":35292,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":35424,"parent_record_offset":35400,"ancestor_record_offset":35292,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":905,"record_offset":35484,"parent_record_offset":35348,"ancestor_record_offset":35292,"inlinee":63900,"source_line":9,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF668
// 首条位置: {"module":910,"inlinee":63080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF999
// 首条位置: {"module":905,"inlinee":63897}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":905,"record_offset":35772,"parent_record_offset":35116,"ancestor_record_offset":35116,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":36740,"parent_record_offset":35116,"ancestor_record_offset":35116,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":905,"record_offset":36676,"parent_record_offset":35116,"ancestor_record_offset":35116,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":905,"record_offset":38016,"parent_record_offset":35116,"ancestor_record_offset":35116,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":38296,"parent_record_offset":35116,"ancestor_record_offset":35116,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":35176,"parent_record_offset":35116,"ancestor_record_offset":35116,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":35800,"parent_record_offset":35116,"ancestor_record_offset":35116,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":36940,"parent_record_offset":36916,"ancestor_record_offset":35116,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":36964,"parent_record_offset":36940,"ancestor_record_offset":35116,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":36988,"parent_record_offset":36964,"ancestor_record_offset":35116,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":36916,"parent_record_offset":36852,"ancestor_record_offset":35116,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":36884,"parent_record_offset":36852,"ancestor_record_offset":35116,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":36852,"parent_record_offset":36824,"ancestor_record_offset":35116,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":36824,"parent_record_offset":36796,"ancestor_record_offset":35116,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":35456,"parent_record_offset":35348,"ancestor_record_offset":35116,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":37072,"parent_record_offset":37048,"ancestor_record_offset":35116,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":905,"record_offset":35540,"parent_record_offset":35516,"ancestor_record_offset":35116,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":905,"record_offset":37856,"parent_record_offset":37828,"ancestor_record_offset":35116,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":36768,"parent_record_offset":36740,"ancestor_record_offset":35116,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":35564,"parent_record_offset":35540,"ancestor_record_offset":35116,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":905,"record_offset":35592,"parent_record_offset":35564,"ancestor_record_offset":35116,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":905,"record_offset":35516,"parent_record_offset":35484,"ancestor_record_offset":35116,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":905,"record_offset":38088,"parent_record_offset":38064,"ancestor_record_offset":35116,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":38064,"parent_record_offset":38040,"ancestor_record_offset":35116,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":35116,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":38112,"parent_record_offset":38088,"ancestor_record_offset":35116,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":35264,"parent_record_offset":35212,"ancestor_record_offset":35116,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":36184,"parent_record_offset":36160,"ancestor_record_offset":35116,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":36636,"parent_record_offset":36612,"ancestor_record_offset":35116,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35400,"parent_record_offset":35348,"ancestor_record_offset":35116,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35884,"parent_record_offset":35840,"ancestor_record_offset":35116,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35928,"parent_record_offset":35884,"ancestor_record_offset":35116,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35968,"parent_record_offset":35928,"ancestor_record_offset":35116,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35212,"parent_record_offset":35176,"ancestor_record_offset":35116,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35840,"parent_record_offset":35800,"ancestor_record_offset":35116,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":35348,"parent_record_offset":35320,"ancestor_record_offset":35116,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":905,"record_offset":35320,"parent_record_offset":35292,"ancestor_record_offset":35116,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":35424,"parent_record_offset":35400,"ancestor_record_offset":35116,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":905,"record_offset":35996,"parent_record_offset":35968,"ancestor_record_offset":35116,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":905,"record_offset":35484,"parent_record_offset":35348,"ancestor_record_offset":35116,"inlinee":63900,"source_line":9,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":905,"record_offset":35292,"parent_record_offset":35264,"ancestor_record_offset":35116,"inlinee":63899,"source_line":9,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF99D
// 首条位置: {"module":905,"inlinee":63901}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":39016,"parent_record_offset":38664,"ancestor_record_offset":38664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":38712,"parent_record_offset":38664,"ancestor_record_offset":38664,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":39316,"parent_record_offset":39292,"ancestor_record_offset":38664,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":39340,"parent_record_offset":39316,"ancestor_record_offset":38664,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":39364,"parent_record_offset":39340,"ancestor_record_offset":38664,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":905,"record_offset":39292,"parent_record_offset":39228,"ancestor_record_offset":38664,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":39260,"parent_record_offset":39228,"ancestor_record_offset":38664,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":39228,"parent_record_offset":39200,"ancestor_record_offset":38664,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":905,"record_offset":39200,"parent_record_offset":39172,"ancestor_record_offset":38664,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":905,"record_offset":39040,"parent_record_offset":39016,"ancestor_record_offset":38664,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":905,"record_offset":39836,"parent_record_offset":39800,"ancestor_record_offset":38664,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":905,"record_offset":39064,"parent_record_offset":39040,"ancestor_record_offset":38664,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":905,"record_offset":38796,"parent_record_offset":38740,"ancestor_record_offset":38664,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":38820,"parent_record_offset":38796,"ancestor_record_offset":38664,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":905,"record_offset":38976,"parent_record_offset":38952,"ancestor_record_offset":38664,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":905,"record_offset":38740,"parent_record_offset":38712,"ancestor_record_offset":38664,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/00905.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFA33
// 首条位置: {"module":910,"inlinee":64051}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8032,"parent_record_offset":7996,"ancestor_record_offset":7996,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8360,"parent_record_offset":7996,"ancestor_record_offset":7996,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8288,"parent_record_offset":7996,"ancestor_record_offset":7996,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":7996,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":7996,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":910,"record_offset":8176,"parent_record_offset":8152,"ancestor_record_offset":7996,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":910,"record_offset":8228,"parent_record_offset":8200,"ancestor_record_offset":7996,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":7996,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":7996,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":7996,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":7996,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":910,"record_offset":8128,"parent_record_offset":8104,"ancestor_record_offset":7996,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":910,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":7996,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":910,"record_offset":1692,"rva":16415904,"va":5385125024}
// 编译实例 RVAs: 0xfa7ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<u8> as std::io::copy::BufferedWriterSpec>::buffer_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-362378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e87678faff","size":5,"soft_fail":false,"target_va":5384763680,"va":5385126053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-467727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8f1dcf8ff","size":5,"soft_fail":false,"target_va":5384659264,"va":5385126986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-435973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8fb58f9ff","size":5,"soft_fail":false,"target_va":5384689616,"va":5385125584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-423382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e82a8af9ff","size":5,"soft_fail":false,"target_va":5384703536,"va":5385126913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-469224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e818d7f8ff","size":5,"soft_fail":false,"target_va":5384656240,"va":5385125459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-469156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e85cd7f8ff","size":5,"soft_fail":false,"target_va":5384656512,"va":5385125663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-13230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e852ccffff","size":5,"soft_fail":false,"target_va":5385113168,"va":5385126393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-9447301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e87bd86fff","size":5,"soft_fail":false,"target_va":5375678688,"va":5385125984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-9447996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8c4d56fff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385126695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16403784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8b8b205ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5385125971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15897120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8e06d0dff","size":5,"soft_fail":false,"target_va":5369231648,"va":5385128763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15787032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8e81b0fff","size":5,"soft_fail":false,"target_va":5369338848,"va":5385125875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15765688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8486f0fff","size":5,"soft_fail":false,"target_va":5369364384,"va":5385130067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13300263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8d90d35ff","size":5,"soft_fail":false,"target_va":5371827984,"va":5385128242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa7a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8d5fcffff","size":5,"soft_fail":false,"target_va":5385124496,"va":5385125302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ed25f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15899748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8649cf200","size":5,"soft_fail":false,"target_va":5401028080,"va":5385128327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ee2430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15967084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e86ca3f300","size":5,"soft_fail":false,"target_va":5401093168,"va":5385126079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31224006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8c670dc01","size":5,"soft_fail":false,"target_va":5416350608,"va":5385126597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31430155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e80b96df01","size":5,"soft_fail":false,"target_va":5416557744,"va":5385127584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31931678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e81e3de701","size":5,"soft_fail":false,"target_va":5417057056,"va":5385125373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31493689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8398ee001","size":5,"soft_fail":false,"target_va":5416620794,"va":5385127100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9427037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8a32770ff","size":5,"soft_fail":false,"target_va":5375700864,"va":5385127896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6c4ab46d71a042a1 (.llvm.8279917044267982437)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31496444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8fc98e001","size":5,"soft_fail":false,"target_va":5416623536,"va":5385127087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31497946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8da9ee001","size":5,"soft_fail":false,"target_va":5416625104,"va":5385127153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8e5e9ffff","size":5,"soft_fail":false,"target_va":5385121616,"va":5385127270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e872e5ffff","size":5,"soft_fail":false,"target_va":5385119680,"va":5385126473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8badaffff","size":5,"soft_fail":false,"target_va":5385116768,"va":5385126305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31493150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e81e8ce001","size":5,"soft_fail":false,"target_va":5416621600,"va":5385128445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$theme_fetcher..checked_zip..FolderChecksumInfo$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-244695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e82944fcff","size":5,"soft_fail":false,"target_va":5384883728,"va":5385128418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..ZipFile$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-247828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8ec37fcff","size":5,"soft_fail":false,"target_va":5384878928,"va":5385126751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31493133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e80d8ce001","size":5,"soft_fail":false,"target_va":5416621632,"va":5385128494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-296070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e87a7bfbff","size":5,"soft_fail":false,"target_va":5384830080,"va":5385126145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::ser::SerializeMap::serialize_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15968122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e87aa7f300","size":5,"soft_fail":false,"target_va":5401095680,"va":5385127553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-373658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e8664cfaff","size":5,"soft_fail":false,"target_va":5384752464,"va":5385126117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::Write::write_all
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15968362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e86aa8f300","size":5,"soft_fail":false,"target_va":5401096672,"va":5385128305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-400800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e860e2f9ff","size":5,"soft_fail":false,"target_va":5384724912,"va":5385125707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::ZipFile::enclosed_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15899919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385125024,"raw_hex":"e80f9df200","size":5,"soft_fail":false,"target_va":5401025344,"va":5385125420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*41032993(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5385125024,"raw_hex":"ff15211d7202","size":6,"soft_fail":false,"va":5385128177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":910,"record_offset":3984,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":910,"record_offset":4640,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":5112,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":3268,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$27::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":1976,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7206,"source_line":4236,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":910,"record_offset":6916,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":910,"record_offset":2032,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":33662,"source_line":1010,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":910,"record_offset":2180,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":910,"record_offset":2308,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":910,"record_offset":3948,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":910,"record_offset":5340,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":910,"record_offset":2276,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":910,"record_offset":7028,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":910,"record_offset":2672,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":7556,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":8445,"source_line":2213,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":910,"record_offset":5992,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":64050,"source_line":712,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":910,"record_offset":2884,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":910,"record_offset":3780,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] std::io::copy::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":910,"record_offset":3664,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":25036,"source_line":62,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":910,"record_offset":2996,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":910,"record_offset":2336,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":910,"record_offset":3836,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":10684,"source_line":2452,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] zip::read::ZipFile::name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":910,"record_offset":2364,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":45541,"source_line":858,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] zip::read::ZipFile::size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":910,"record_offset":3208,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":64049,"source_line":926,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_direct] zip::read::zip_archive::ZipArchive::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":910,"record_offset":2124,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":45539,"source_line":483,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":910,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":1692,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":910,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":1692,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":910,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":1692,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":910,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":1692,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":910,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":1692,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":910,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":1692,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":910,"record_offset":3532,"parent_record_offset":3384,"ancestor_record_offset":1692,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":910,"record_offset":4972,"parent_record_offset":4916,"ancestor_record_offset":1692,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":910,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":1692,"inlinee":6483,"source_line":349,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":2556,"parent_record_offset":2528,"ancestor_record_offset":1692,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":3324,"parent_record_offset":3296,"ancestor_record_offset":1692,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":4944,"parent_record_offset":4916,"ancestor_record_offset":1692,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":1692,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":2612,"parent_record_offset":2584,"ancestor_record_offset":1692,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":2584,"parent_record_offset":2556,"ancestor_record_offset":1692,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":3384,"parent_record_offset":3352,"ancestor_record_offset":1692,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":3352,"parent_record_offset":3324,"ancestor_record_offset":1692,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":1692,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":910,"record_offset":3420,"parent_record_offset":3384,"ancestor_record_offset":1692,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":910,"record_offset":4056,"parent_record_offset":4020,"ancestor_record_offset":1692,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":910,"record_offset":4092,"parent_record_offset":4056,"ancestor_record_offset":1692,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":910,"record_offset":4128,"parent_record_offset":4092,"ancestor_record_offset":1692,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":910,"record_offset":4020,"parent_record_offset":3984,"ancestor_record_offset":1692,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":910,"record_offset":2456,"parent_record_offset":2392,"ancestor_record_offset":1692,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":910,"record_offset":4728,"parent_record_offset":4676,"ancestor_record_offset":1692,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":910,"record_offset":4676,"parent_record_offset":4640,"ancestor_record_offset":1692,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":910,"record_offset":10828,"parent_record_offset":10788,"ancestor_record_offset":1692,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":910,"record_offset":10788,"parent_record_offset":10760,"ancestor_record_offset":1692,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":1692,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":5184,"parent_record_offset":5140,"ancestor_record_offset":1692,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":2528,"parent_record_offset":2492,"ancestor_record_offset":1692,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":2492,"parent_record_offset":2456,"ancestor_record_offset":1692,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":1692,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":1692,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1692,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":5140,"parent_record_offset":5112,"ancestor_record_offset":1692,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":7624,"parent_record_offset":7556,"ancestor_record_offset":1692,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":3296,"parent_record_offset":3268,"ancestor_record_offset":1692,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":910,"record_offset":9788,"parent_record_offset":9764,"ancestor_record_offset":1692,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":910,"record_offset":8176,"parent_record_offset":8152,"ancestor_record_offset":1692,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":910,"record_offset":6988,"parent_record_offset":6948,"ancestor_record_offset":1692,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":910,"record_offset":6948,"parent_record_offset":6916,"ancestor_record_offset":1692,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":910,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":1692,"inlinee":4748,"source_line":254,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":910,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":1692,"inlinee":4746,"source_line":50,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":910,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":1692,"inlinee":4753,"source_line":716,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":910,"record_offset":13424,"parent_record_offset":13400,"ancestor_record_offset":1692,"inlinee":4750,"source_line":72,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":910,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":1692,"inlinee":4751,"source_line":76,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":910,"record_offset":2084,"parent_record_offset":2056,"ancestor_record_offset":1692,"inlinee":33665,"source_line":477,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":910,"record_offset":2056,"parent_record_offset":2032,"ancestor_record_offset":1692,"inlinee":33663,"source_line":1001,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":910,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":1692,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":910,"record_offset":2236,"parent_record_offset":2208,"ancestor_record_offset":1692,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":910,"record_offset":2784,"parent_record_offset":2756,"ancestor_record_offset":1692,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":910,"record_offset":2936,"parent_record_offset":2884,"ancestor_record_offset":1692,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":910,"record_offset":4756,"parent_record_offset":4676,"ancestor_record_offset":1692,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":910,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":1692,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":910,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":1692,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":910,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":1692,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":910,"record_offset":10572,"parent_record_offset":10544,"ancestor_record_offset":1692,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":910,"record_offset":4564,"parent_record_offset":4128,"ancestor_record_offset":1692,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":910,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":1692,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=80
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":910,"record_offset":3088,"parent_record_offset":2996,"ancestor_record_offset":1692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":910,"record_offset":10880,"parent_record_offset":10828,"ancestor_record_offset":1692,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":910,"record_offset":5304,"parent_record_offset":5140,"ancestor_record_offset":1692,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":910,"record_offset":4784,"parent_record_offset":4676,"ancestor_record_offset":1692,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":910,"record_offset":3048,"parent_record_offset":3020,"ancestor_record_offset":1692,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":910,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":1692,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":910,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":1692,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":910,"record_offset":2756,"parent_record_offset":2728,"ancestor_record_offset":1692,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":910,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":1692,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":910,"record_offset":2700,"parent_record_offset":2672,"ancestor_record_offset":1692,"inlinee":22388,"source_line":603,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":910,"record_offset":10912,"parent_record_offset":10828,"ancestor_record_offset":1692,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":910,"record_offset":10948,"parent_record_offset":10788,"ancestor_record_offset":1692,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":910,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":1692,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":1692,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":1692,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":7940,"parent_record_offset":7900,"ancestor_record_offset":1692,"inlinee":8447,"source_line":28,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":1692,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":1692,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8032,"parent_record_offset":7996,"ancestor_record_offset":1692,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":1692,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8360,"parent_record_offset":7996,"ancestor_record_offset":1692,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":8288,"parent_record_offset":7996,"ancestor_record_offset":1692,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":910,"record_offset":7900,"parent_record_offset":7556,"ancestor_record_offset":1692,"inlinee":8446,"source_line":2177,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":910,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":1692,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":910,"record_offset":2964,"parent_record_offset":2884,"ancestor_record_offset":1692,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":910,"record_offset":2908,"parent_record_offset":2884,"ancestor_record_offset":1692,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":910,"record_offset":6024,"parent_record_offset":5992,"ancestor_record_offset":1692,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":910,"record_offset":6148,"parent_record_offset":5992,"ancestor_record_offset":1692,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":910,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":1692,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Read::read_to_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":910,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":1692,"inlinee":25175,"source_line":935,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::generic_copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":910,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":1692,"inlinee":25037,"source_line":77,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::impl$6::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\copy.rs
//     evidence={"module":910,"record_offset":3716,"parent_record_offset":3688,"ancestor_record_offset":1692,"inlinee":25039,"source_line":266,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":910,"record_offset":8128,"parent_record_offset":8104,"ancestor_record_offset":1692,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":910,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":1692,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":910,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":1692,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":910,"record_offset":6116,"parent_record_offset":6024,"ancestor_record_offset":1692,"inlinee":39221,"source_line":230,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":910,"record_offset":6060,"parent_record_offset":6024,"ancestor_record_offset":1692,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":910,"record_offset":6088,"parent_record_offset":6024,"ancestor_record_offset":1692,"inlinee":7544,"source_line":218,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":910,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":1692,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":910,"record_offset":7996,"parent_record_offset":7900,"ancestor_record_offset":1692,"inlinee":64051,"source_line":9,"archive":"_pdb/C/modules/00910.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::verify_folder_checksums
// 记录形式: compiled_procedure; instances=1
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":893,"record_offset":1692,"rva":16177056,"va":5384886176}
// 编译实例 RVAs: 0xf6d7a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-227081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8f788fcff","size":5,"soft_fail":false,"target_va":5384659264,"va":5384886340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-184045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e81331fdff","size":5,"soft_fail":false,"target_va":5384703536,"va":5384887576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-230005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e88b7dfcff","size":5,"soft_fail":false,"target_va":5384656240,"va":5384886240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t226191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e88f730300","size":5,"soft_fail":false,"target_va":5385113168,"va":5384886972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11654681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e819d6b100","size":5,"soft_fail":false,"target_va":5396542816,"va":5384888130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=29
//     evidence={"assembly":"\tcallq\t-9207596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8d48073ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5384886295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15657856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8801411ff","size":5,"soft_fail":false,"target_va":5369231648,"va":5384889499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15549265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8afbc12ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5384888108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15545905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8cfc912ff","size":5,"soft_fail":false,"target_va":5369343504,"va":5384889404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972470
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6273944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e86844a0ff","size":5,"soft_fail":false,"target_va":5378614384,"va":5384888323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9fee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t206362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e81a260300","size":5,"soft_fail":false,"target_va":5385092832,"va":5384886465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ed24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16140163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e88347f600","size":5,"soft_fail":false,"target_va":5401027760,"va":5384887592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32170009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e819e0ea01","size":5,"soft_fail":false,"target_va":5417057040,"va":5384887026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-93663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e82192feff","size":5,"soft_fail":false,"target_va":5384794864,"va":5384888522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6c4ab46d71a042a1 (.llvm.8279917044267982437)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31737111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e81745e401","size":5,"soft_fail":false,"target_va":5416623536,"va":5384886420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hb429360875c1aa7a (.llvm.8279917044267982437)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31737548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8cc46e401","size":5,"soft_fail":false,"target_va":5416624144,"va":5384886591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31735939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e88340e401","size":5,"soft_fail":false,"target_va":5416623024,"va":5384887080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31736557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8ed42e401","size":5,"soft_fail":false,"target_va":5416625264,"va":5384888702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t234731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8eb940300","size":5,"soft_fail":false,"target_va":5385121616,"va":5384886880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t232852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8948d0300","size":5,"soft_fail":false,"target_va":5385119680,"va":5384886823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t228908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e82c7e0300","size":5,"soft_fail":false,"target_va":5385116768,"va":5384887855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<memmap2::Mmap,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e808010000","size":5,"soft_fail":false,"target_va":5384890208,"va":5384889939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31732566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e85633e401","size":5,"soft_fail":false,"target_va":5416621632,"va":5384889061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11655278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e86ed8b100","size":5,"soft_fail":false,"target_va":5396542944,"va":5384887661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2764047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e8f1d2d5ff","size":5,"soft_fail":false,"target_va":5382123584,"va":5384887626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-134336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384886176,"raw_hex":"e840f3fdff","size":5,"soft_fail":false,"target_va":5384751936,"va":5384886267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rdi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384886176,"raw_hex":"ffd7","size":2,"soft_fail":false,"va":5384887712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":893,"record_offset":9440,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":893,"record_offset":3220,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":893,"record_offset":3016,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":5952,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":893,"record_offset":4132,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":893,"record_offset":2676,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":893,"record_offset":2924,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":33662,"source_line":1010,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":893,"record_offset":3692,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":893,"record_offset":9524,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":893,"record_offset":6444,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":893,"record_offset":3848,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":8588,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":8050,"source_line":768,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":6640,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":9780,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":893,"record_offset":3128,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":893,"record_offset":6228,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":15362,"source_line":579,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":893,"record_offset":6112,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":893,"record_offset":1928,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":893,"record_offset":1900,"parent_record_offset":1692,"ancestor_record_offset":1692,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":893,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":1692,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":893,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":1692,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":893,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":1692,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":893,"record_offset":2160,"parent_record_offset":2108,"ancestor_record_offset":1692,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":893,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":1692,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":893,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1692,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":893,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1692,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":893,"record_offset":2084,"parent_record_offset":2064,"ancestor_record_offset":1692,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":893,"record_offset":3308,"parent_record_offset":3256,"ancestor_record_offset":1692,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":893,"record_offset":3256,"parent_record_offset":3220,"ancestor_record_offset":1692,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":893,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":1692,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":893,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":1692,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":10652,"parent_record_offset":10624,"ancestor_record_offset":1692,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":10624,"parent_record_offset":10600,"ancestor_record_offset":1692,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":1692,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":893,"record_offset":10736,"parent_record_offset":10708,"ancestor_record_offset":1692,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":893,"record_offset":3748,"parent_record_offset":3720,"ancestor_record_offset":1692,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":893,"record_offset":2764,"parent_record_offset":2716,"ancestor_record_offset":1692,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":893,"record_offset":2716,"parent_record_offset":2676,"ancestor_record_offset":1692,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":893,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":1692,"inlinee":4748,"source_line":254,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":893,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":1692,"inlinee":4746,"source_line":50,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":893,"record_offset":12264,"parent_record_offset":12236,"ancestor_record_offset":1692,"inlinee":4753,"source_line":716,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":893,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":1692,"inlinee":4750,"source_line":72,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":893,"record_offset":12236,"parent_record_offset":12212,"ancestor_record_offset":1692,"inlinee":4751,"source_line":76,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":893,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":1692,"inlinee":33665,"source_line":477,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":893,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":1692,"inlinee":33663,"source_line":1001,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":893,"record_offset":3336,"parent_record_offset":3256,"ancestor_record_offset":1692,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":893,"record_offset":10708,"parent_record_offset":10680,"ancestor_record_offset":1692,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":893,"record_offset":10680,"parent_record_offset":10652,"ancestor_record_offset":1692,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":893,"record_offset":4968,"parent_record_offset":4940,"ancestor_record_offset":1692,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":893,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":1692,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":893,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":1692,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":893,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":1692,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=87
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":893,"record_offset":1964,"parent_record_offset":1928,"ancestor_record_offset":1692,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":893,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":1692,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":893,"record_offset":10784,"parent_record_offset":10760,"ancestor_record_offset":1692,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":893,"record_offset":3188,"parent_record_offset":3128,"ancestor_record_offset":1692,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":893,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":1692,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":893,"record_offset":6052,"parent_record_offset":5952,"ancestor_record_offset":1692,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":893,"record_offset":3160,"parent_record_offset":3128,"ancestor_record_offset":1692,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":893,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":1692,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":3364,"parent_record_offset":3256,"ancestor_record_offset":1692,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":6400,"parent_record_offset":6256,"ancestor_record_offset":1692,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":893,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":1692,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":893,"record_offset":3776,"parent_record_offset":3748,"ancestor_record_offset":1692,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":893,"record_offset":3804,"parent_record_offset":3776,"ancestor_record_offset":1692,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":893,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":1692,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":893,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":1692,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":893,"record_offset":6288,"parent_record_offset":6256,"ancestor_record_offset":1692,"inlinee":11999,"source_line":228,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":893,"record_offset":6256,"parent_record_offset":6228,"ancestor_record_offset":1692,"inlinee":12011,"source_line":338,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":893,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":1692,"inlinee":12001,"source_line":229,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":893,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":1692,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":893,"record_offset":6196,"parent_record_offset":6112,"ancestor_record_offset":1692,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":893,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":1692,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":893,"record_offset":6164,"parent_record_offset":6136,"ancestor_record_offset":1692,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/00893.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::verify_folder_checksums::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19652
// 首条位置: {"module":2329,"inlinee":104018}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2329,"record_offset":540,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02329.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2329,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":520,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02329.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2329,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":520,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02329.jsonl.gz"}
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
// 完整结构及原始类型记录: checked_zip.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] theme_fetcher::checked_zip::FolderChecksumInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<theme_fetcher::checked_zip::FolderChecksumInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<theme_fetcher::checked_zip::FolderChecksumInfo>", "constructor": "serde_json::de::from_slice", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: checked_zip.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: checked_zip.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize
// 信息 ID: a956862fac298dcfc1d222de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::from_slice<theme_fetcher::checked_zip::FolderChecksumInfo>; module=905, inline_record=6456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: bd9920376d4cc8c33054ca56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=905, inline_record=35484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 427e153416619aee505ff2c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=905, inline_record=35292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 4d7c8d1e0dc3adf40f0873fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=910, inline_record=15296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: f17482d3d9fc3d66347094ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=905, inline_record=35116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 104da1513320a9f593138266
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=905, inline_record=38664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::serialize
// 信息 ID: 5dea357b46e7d237fd2104ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >; module=910, inline_record=7996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 信息 ID: d4a05e3941e3b2c4d05a0336
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// [类型关系] theme_fetcher::checked_zip::extract_with_checksums -> ["std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x6a85a2 -> 0xfa7ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::verify_folder_checksums
// 信息 ID: 7bcaf5a6b13861709409136f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x6a5e65 -> 0xf6d7a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::verify_folder_checksums::closure$0
// 信息 ID: f575b88eeebbe7d1533f0bbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::checked_zip::verify_folder_checksums", "captures": "unknown"}
// [闭包环境类型引用] theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0>; module=2329, inline_record=520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
