// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/prefab/config.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: fadf56298c93f08672f42c8078ca88b66257122e5171940ff7b56201209b9fa0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: config.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 539, 543, 1658, 1660, 1805, 2144, 3383, 3385, 3612, 3613, 3614, 3615, 4501, 4577
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=55
// 源码行: 5, 26, 70, 107, 113, 168, 174, 196, 232, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18027
// 首条位置: {"module":2144,"inlinee":98343}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 5, 70, 107, 168, 174, 196, 232, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA40
// 首条位置: {"module":3613,"inlinee":129600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3613,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3613,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":660,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":660,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":21084,"parent_record_offset":21048,"ancestor_record_offset":20936,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 5, 26, 70, 107, 113, 168, 174, 196, 232, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA51
// 首条位置: {"module":3613,"inlinee":129617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::id::impl$1::deserialize_identifier
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":4501,"record_offset":29920,"parent_record_offset":29896,"ancestor_record_offset":29896,"inlinee":9960,"source_line":19,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":30248,"parent_record_offset":30208,"ancestor_record_offset":30208,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":580,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":552,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":552,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":552,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3613,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":552,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3613,"record_offset":980,"parent_record_offset":580,"ancestor_record_offset":552,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":552,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":552,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":776,"parent_record_offset":580,"ancestor_record_offset":552,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":552,"inlinee":129600,"source_line":174,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1716,"inlinee":129590,"source_line":174,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3613,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":552,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":30300,"parent_record_offset":30248,"ancestor_record_offset":30208,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":30796,"parent_record_offset":30248,"ancestor_record_offset":30208,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":552,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":21084,"parent_record_offset":21048,"ancestor_record_offset":20728,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3613,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":552,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":41040,"parent_record_offset":41012,"ancestor_record_offset":40984,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":21356,"parent_record_offset":20768,"ancestor_record_offset":20728,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":636,"parent_record_offset":580,"ancestor_record_offset":552,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":1808,"parent_record_offset":1748,"ancestor_record_offset":1716,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 5, 26, 70, 113, 174, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA52
// 首条位置: {"module":3613,"inlinee":129618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3613,"record_offset":14300,"parent_record_offset":13132,"ancestor_record_offset":13132,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3613,"record_offset":23224,"parent_record_offset":20384,"ancestor_record_offset":20384,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":30888,"parent_record_offset":29440,"ancestor_record_offset":29440,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":23196,"parent_record_offset":20384,"ancestor_record_offset":20384,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":29496,"parent_record_offset":29440,"ancestor_record_offset":29440,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":30944,"parent_record_offset":29440,"ancestor_record_offset":29440,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":13188,"parent_record_offset":13132,"ancestor_record_offset":13132,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":14328,"parent_record_offset":13132,"ancestor_record_offset":13132,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":4501,"record_offset":51640,"parent_record_offset":49232,"ancestor_record_offset":49232,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":31996,"parent_record_offset":31972,"ancestor_record_offset":29440,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":32020,"parent_record_offset":31996,"ancestor_record_offset":29440,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3613,"record_offset":23432,"parent_record_offset":23408,"ancestor_record_offset":20384,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3613,"record_offset":23456,"parent_record_offset":23432,"ancestor_record_offset":20384,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":32044,"parent_record_offset":32020,"ancestor_record_offset":29440,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3613,"record_offset":23480,"parent_record_offset":23456,"ancestor_record_offset":20384,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":31972,"parent_record_offset":31824,"ancestor_record_offset":29440,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3613,"record_offset":23408,"parent_record_offset":23384,"ancestor_record_offset":20384,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":13844,"parent_record_offset":13816,"ancestor_record_offset":13132,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31764,"parent_record_offset":31736,"ancestor_record_offset":29440,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":43244,"parent_record_offset":43216,"ancestor_record_offset":40528,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":23384,"parent_record_offset":23360,"ancestor_record_offset":20384,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":13900,"parent_record_offset":13872,"ancestor_record_offset":13132,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":13872,"parent_record_offset":13844,"ancestor_record_offset":13132,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31824,"parent_record_offset":31792,"ancestor_record_offset":29440,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31792,"parent_record_offset":31764,"ancestor_record_offset":29440,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":23360,"parent_record_offset":23336,"ancestor_record_offset":20384,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31860,"parent_record_offset":31824,"ancestor_record_offset":29440,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31652,"parent_record_offset":31624,"ancestor_record_offset":29440,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31680,"parent_record_offset":31652,"ancestor_record_offset":29440,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31708,"parent_record_offset":31680,"ancestor_record_offset":29440,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31624,"parent_record_offset":31596,"ancestor_record_offset":29440,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4501,"record_offset":31596,"parent_record_offset":31568,"ancestor_record_offset":29440,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3613,"record_offset":14012,"parent_record_offset":13984,"ancestor_record_offset":13132,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3613,"record_offset":13984,"parent_record_offset":13596,"ancestor_record_offset":13132,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":13132,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":13788,"parent_record_offset":13760,"ancestor_record_offset":13132,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4501,"record_offset":31736,"parent_record_offset":31708,"ancestor_record_offset":29440,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":13760,"parent_record_offset":13596,"ancestor_record_offset":13132,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":13132,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":13132,"inlinee":98343,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":20936,"parent_record_offset":20904,"ancestor_record_offset":20384,"inlinee":129600,"source_line":174,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":13720,"parent_record_offset":13684,"ancestor_record_offset":13132,"inlinee":129590,"source_line":174,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":13552,"parent_record_offset":13508,"ancestor_record_offset":13132,"inlinee":129617,"source_line":174,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":13132,"inlinee":129609,"source_line":174,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":15544,"parent_record_offset":15516,"ancestor_record_offset":13132,"inlinee":129613,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":13132,"inlinee":129605,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":13132,"inlinee":129611,"source_line":113,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$4::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":13132,"inlinee":129620,"source_line":113,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":15260,"parent_record_offset":15232,"ancestor_record_offset":13132,"inlinee":129607,"source_line":113,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4501,"record_offset":31884,"parent_record_offset":31860,"ancestor_record_offset":29440,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":4501,"record_offset":29772,"parent_record_offset":29748,"ancestor_record_offset":29440,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":29800,"parent_record_offset":29772,"ancestor_record_offset":29440,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3613,"record_offset":21016,"parent_record_offset":20984,"ancestor_record_offset":20384,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":29440,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":29440,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":29840,"parent_record_offset":29544,"ancestor_record_offset":29440,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":30104,"parent_record_offset":29544,"ancestor_record_offset":29440,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=154
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3613,"record_offset":23264,"parent_record_offset":23224,"ancestor_record_offset":20384,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3613,"record_offset":13468,"parent_record_offset":13384,"ancestor_record_offset":13132,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3613,"record_offset":13432,"parent_record_offset":13384,"ancestor_record_offset":13132,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":14248,"parent_record_offset":13276,"ancestor_record_offset":13132,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":14500,"parent_record_offset":14448,"ancestor_record_offset":13132,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4501,"record_offset":30796,"parent_record_offset":30248,"ancestor_record_offset":29440,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":21048,"parent_record_offset":21016,"ancestor_record_offset":20384,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":21084,"parent_record_offset":21048,"ancestor_record_offset":20384,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3613,"record_offset":13384,"parent_record_offset":13336,"ancestor_record_offset":13132,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3613,"record_offset":20984,"parent_record_offset":20936,"ancestor_record_offset":20384,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":4501,"record_offset":51308,"parent_record_offset":51284,"ancestor_record_offset":49232,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::impl$1::deserialize_identifier
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":4501,"record_offset":29920,"parent_record_offset":29896,"ancestor_record_offset":29440,"inlinee":9960,"source_line":19,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":42144,"parent_record_offset":42120,"ancestor_record_offset":40528,"inlinee":9919,"source_line":361,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":30248,"parent_record_offset":30208,"ancestor_record_offset":29440,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31476,"parent_record_offset":31444,"ancestor_record_offset":29440,"inlinee":7149,"source_line":461,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31444,"parent_record_offset":31412,"ancestor_record_offset":29440,"inlinee":7148,"source_line":471,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":29544,"parent_record_offset":29496,"ancestor_record_offset":29440,"inlinee":9956,"source_line":852,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31008,"parent_record_offset":30944,"ancestor_record_offset":29440,"inlinee":9908,"source_line":873,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_content
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31192,"parent_record_offset":31008,"ancestor_record_offset":29440,"inlinee":9910,"source_line":1050,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_tag_or_content
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":29748,"parent_record_offset":29544,"ancestor_record_offset":29440,"inlinee":9920,"source_line":1109,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::Deserializer::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":34192,"parent_record_offset":33820,"ancestor_record_offset":29440,"inlinee":10096,"source_line":10,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":32344,"parent_record_offset":32312,"ancestor_record_offset":29440,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":32312,"parent_record_offset":32280,"ancestor_record_offset":29440,"inlinee":7156,"source_line":25,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":13336,"parent_record_offset":13276,"ancestor_record_offset":13132,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":13596,"parent_record_offset":13552,"ancestor_record_offset":13132,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":13276,"parent_record_offset":13232,"ancestor_record_offset":13132,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":13132,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":13232,"parent_record_offset":13188,"ancestor_record_offset":13132,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":13132,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":21356,"parent_record_offset":20768,"ancestor_record_offset":20384,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":14208,"parent_record_offset":13596,"ancestor_record_offset":13132,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":13684,"parent_record_offset":13596,"ancestor_record_offset":13132,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":13508,"parent_record_offset":13276,"ancestor_record_offset":13132,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":13132,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":51072,"parent_record_offset":51048,"ancestor_record_offset":49232,"inlinee":83339,"source_line":1515,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":42120,"parent_record_offset":42096,"ancestor_record_offset":40528,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":31568,"parent_record_offset":31540,"ancestor_record_offset":29440,"inlinee":7668,"source_line":592,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":23108,"parent_record_offset":23080,"ancestor_record_offset":20384,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4501,"record_offset":51688,"parent_record_offset":51664,"ancestor_record_offset":49232,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":4501,"record_offset":51664,"parent_record_offset":51640,"ancestor_record_offset":49232,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$13::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 5
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x157A3
// 首条位置: {"module":1805,"inlinee":87971}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1805,"record_offset":2044,"parent_record_offset":2008,"ancestor_record_offset":2008,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1805,"record_offset":2584,"parent_record_offset":2556,"ancestor_record_offset":2008,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1805,"record_offset":2484,"parent_record_offset":2280,"ancestor_record_offset":2008,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1805,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":2008,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1805,"record_offset":2612,"parent_record_offset":2556,"ancestor_record_offset":2008,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1805,"record_offset":2192,"parent_record_offset":2156,"ancestor_record_offset":2008,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1805,"record_offset":2312,"parent_record_offset":2280,"ancestor_record_offset":2008,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1805,"record_offset":2400,"parent_record_offset":2280,"ancestor_record_offset":2008,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1805,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":2008,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":2008,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1805,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":2008,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1805,"record_offset":2220,"parent_record_offset":2192,"ancestor_record_offset":2008,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":2556,"parent_record_offset":2156,"ancestor_record_offset":2008,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1805,"record_offset":2072,"parent_record_offset":2044,"ancestor_record_offset":2008,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1805,"record_offset":2248,"parent_record_offset":2220,"ancestor_record_offset":2008,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1805,"record_offset":2280,"parent_record_offset":2248,"ancestor_record_offset":2008,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1805,"record_offset":2156,"parent_record_offset":2128,"ancestor_record_offset":2008,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1805,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":2008,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01805.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$1::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x157AA
// 首条位置: {"module":1805,"inlinee":87978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 26, 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3612,"record_offset":2532,"rva":41016176,"va":5409725296}
// 编译实例 RVAs: 0x271db70, 0x271ddd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6829777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409725296,"raw_hex":"e8d1366800","size":5,"soft_fail":false,"target_va":5416555360,"va":5409725578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3612,"record_offset":2708,"parent_record_offset":2532,"ancestor_record_offset":2532,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3612,"record_offset":2728,"parent_record_offset":2708,"ancestor_record_offset":2532,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3612,"record_offset":2748,"parent_record_offset":2728,"ancestor_record_offset":2532,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3612,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":2532,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 26, 70, 113, 174, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA49
// 首条位置: {"module":3613,"inlinee":129609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":14424,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":31996,"parent_record_offset":31972,"ancestor_record_offset":31348,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":32020,"parent_record_offset":31996,"ancestor_record_offset":31348,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":32044,"parent_record_offset":32020,"ancestor_record_offset":31348,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":31972,"parent_record_offset":31824,"ancestor_record_offset":31348,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31764,"parent_record_offset":31736,"ancestor_record_offset":31348,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31824,"parent_record_offset":31792,"ancestor_record_offset":31348,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31792,"parent_record_offset":31764,"ancestor_record_offset":31348,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":31860,"parent_record_offset":31824,"ancestor_record_offset":31348,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31652,"parent_record_offset":31624,"ancestor_record_offset":31348,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31680,"parent_record_offset":31652,"ancestor_record_offset":31348,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31708,"parent_record_offset":31680,"ancestor_record_offset":31348,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":31624,"parent_record_offset":31596,"ancestor_record_offset":31348,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4501,"record_offset":31596,"parent_record_offset":31568,"ancestor_record_offset":31348,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4501,"record_offset":31736,"parent_record_offset":31708,"ancestor_record_offset":31348,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":14424,"inlinee":98343,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4501,"record_offset":31884,"parent_record_offset":31860,"ancestor_record_offset":31348,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":31932,"parent_record_offset":31908,"ancestor_record_offset":31348,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":31348,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":14500,"parent_record_offset":14448,"ancestor_record_offset":14424,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":42144,"parent_record_offset":42120,"ancestor_record_offset":42072,"inlinee":9919,"source_line":361,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31476,"parent_record_offset":31444,"ancestor_record_offset":31348,"inlinee":7149,"source_line":461,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31444,"parent_record_offset":31412,"ancestor_record_offset":31348,"inlinee":7148,"source_line":471,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":32344,"parent_record_offset":32312,"ancestor_record_offset":32216,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":32312,"parent_record_offset":32280,"ancestor_record_offset":32216,"inlinee":7156,"source_line":25,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":31540,"parent_record_offset":31476,"ancestor_record_offset":31348,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":42120,"parent_record_offset":42096,"ancestor_record_offset":42072,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":31568,"parent_record_offset":31540,"ancestor_record_offset":31348,"inlinee":7668,"source_line":592,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":31412,"parent_record_offset":31380,"ancestor_record_offset":31348,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 26, 70, 113, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA4D
// 首条位置: {"module":3613,"inlinee":129613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":15572,"parent_record_offset":15544,"ancestor_record_offset":15544,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":15668,"parent_record_offset":15544,"ancestor_record_offset":15544,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4501,"record_offset":42824,"parent_record_offset":42800,"ancestor_record_offset":42776,"inlinee":98343,"source_line":70,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":15628,"parent_record_offset":15572,"ancestor_record_offset":15544,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":15600,"parent_record_offset":15572,"ancestor_record_offset":15544,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 26, 70, 113, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA45
// 首条位置: {"module":3613,"inlinee":129605}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":15852,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":15964,"parent_record_offset":15852,"ancestor_record_offset":15852,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":36200,"parent_record_offset":36176,"ancestor_record_offset":35512,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":36224,"parent_record_offset":36200,"ancestor_record_offset":35512,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":36248,"parent_record_offset":36224,"ancestor_record_offset":35512,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4501,"record_offset":36176,"parent_record_offset":36028,"ancestor_record_offset":35512,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":35964,"parent_record_offset":35932,"ancestor_record_offset":35512,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":36028,"parent_record_offset":35996,"ancestor_record_offset":35512,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":35996,"parent_record_offset":35964,"ancestor_record_offset":35512,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4501,"record_offset":36064,"parent_record_offset":36028,"ancestor_record_offset":35512,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":35836,"parent_record_offset":35804,"ancestor_record_offset":35512,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":35868,"parent_record_offset":35836,"ancestor_record_offset":35512,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":35900,"parent_record_offset":35868,"ancestor_record_offset":35512,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4501,"record_offset":35804,"parent_record_offset":35772,"ancestor_record_offset":35512,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4501,"record_offset":35772,"parent_record_offset":35740,"ancestor_record_offset":35512,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4501,"record_offset":35932,"parent_record_offset":35900,"ancestor_record_offset":35512,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":4501,"record_offset":36088,"parent_record_offset":36064,"ancestor_record_offset":35512,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":4501,"record_offset":36136,"parent_record_offset":36112,"ancestor_record_offset":35512,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4501,"record_offset":36112,"parent_record_offset":36088,"ancestor_record_offset":35512,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":15928,"parent_record_offset":15876,"ancestor_record_offset":15852,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":35640,"parent_record_offset":35608,"ancestor_record_offset":35512,"inlinee":7149,"source_line":461,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4501,"record_offset":35608,"parent_record_offset":35576,"ancestor_record_offset":35512,"inlinee":7148,"source_line":471,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":36548,"parent_record_offset":36516,"ancestor_record_offset":36420,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4501,"record_offset":36516,"parent_record_offset":36484,"ancestor_record_offset":36420,"inlinee":7156,"source_line":25,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4501,"record_offset":35708,"parent_record_offset":35640,"ancestor_record_offset":35512,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":15900,"parent_record_offset":15876,"ancestor_record_offset":15852,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4501,"record_offset":35740,"parent_record_offset":35708,"ancestor_record_offset":35512,"inlinee":7668,"source_line":592,"archive":"_pdb/C/modules/04501.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":23108,"parent_record_offset":23080,"ancestor_record_offset":23052,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 26, 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA4B
// 首条位置: {"module":3613,"inlinee":129611}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":14984,"parent_record_offset":14960,"ancestor_record_offset":14960,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":15072,"parent_record_offset":14960,"ancestor_record_offset":14960,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":15036,"parent_record_offset":14984,"ancestor_record_offset":14960,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":15008,"parent_record_offset":14984,"ancestor_record_offset":14960,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$14::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC0E
// 首条位置: {"module":3385,"inlinee":121870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4248,"parent_record_offset":4172,"ancestor_record_offset":4172,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":6200,"parent_record_offset":4172,"ancestor_record_offset":4172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":4172,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":4172,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4172,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":4172,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":4172,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4172,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4360,"parent_record_offset":4316,"ancestor_record_offset":4172,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4316,"parent_record_offset":4276,"ancestor_record_offset":4172,"inlinee":121872,"source_line":70,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4460,"parent_record_offset":4428,"ancestor_record_offset":4172,"inlinee":33159,"source_line":107,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5236,"parent_record_offset":5200,"ancestor_record_offset":4172,"inlinee":121874,"source_line":113,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5896,"parent_record_offset":5856,"ancestor_record_offset":4172,"inlinee":121876,"source_line":174,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5980,"parent_record_offset":5932,"ancestor_record_offset":4172,"inlinee":121878,"source_line":196,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":4172,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":4172,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3385,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":4172,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":4536,"parent_record_offset":4316,"ancestor_record_offset":4172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4172,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$2::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC05
// 首条位置: {"module":3385,"inlinee":121861}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$3::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC07
// 首条位置: {"module":3385,"inlinee":121863}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":2300,"parent_record_offset":2264,"ancestor_record_offset":2264,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4004,"parent_record_offset":3968,"ancestor_record_offset":3924,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$17::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC10
// 首条位置: {"module":3385,"inlinee":121872}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":4360,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":4536,"parent_record_offset":4316,"ancestor_record_offset":4316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":4316,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":4316,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4316,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3385,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":4316,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":4316,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3385,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4316,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":4460,"parent_record_offset":4428,"ancestor_record_offset":4316,"inlinee":33159,"source_line":107,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3385,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":4316,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":4316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$4::concrete_default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA71
// 首条位置: {"module":3615,"inlinee":129649}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$5::inherit_from
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3614,"record_offset":1096,"rva":41038032,"va":5409747152}
// 编译实例 RVAs: 0x27230d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-24951365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409747152,"raw_hex":"e8bb4583fe","size":5,"soft_fail":false,"target_va":5384795984,"va":5409747344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3614,"record_offset":1224,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3614,"record_offset":1252,"parent_record_offset":1224,"ancestor_record_offset":1096,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3614,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":1096,"inlinee":33159,"source_line":107,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3614,"record_offset":1356,"parent_record_offset":1324,"ancestor_record_offset":1096,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=6
// 源码行: 107, 113, 168, 174, 196, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3612,"record_offset":896,"rva":41015072,"va":5409724192}
// 编译实例 RVAs: 0x271d720, 0x271d820, 0x271dca0, 0x271de90, 0x271dfa0, 0x271e130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6830220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409724448,"raw_hex":"e88c386800","size":5,"soft_fail":false,"target_va":5416555360,"va":5409725135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6831492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409724192,"raw_hex":"e8843d6800","size":5,"soft_fail":false,"target_va":5416555792,"va":5409724295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-34045660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409724192,"raw_hex":"e82481f8fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409724359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25105367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409724192,"raw_hex":"e829ec80fe","size":5,"soft_fail":false,"target_va":5384618896,"va":5409724258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3612,"record_offset":1076,"parent_record_offset":896,"ancestor_record_offset":896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3612,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":896,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3612,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":896,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3612,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":896,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3612,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":896,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3612,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":896,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3612,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":896,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3612,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":896,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 107, 168, 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA3F
// 首条位置: {"module":3613,"inlinee":129599}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":6064,"parent_record_offset":6012,"ancestor_record_offset":6012,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":6204,"parent_record_offset":6012,"ancestor_record_offset":6012,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$52::unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":6100,"parent_record_offset":6012,"ancestor_record_offset":6012,"inlinee":6485,"source_line":2849,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":6012,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_unit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":6012,"inlinee":6489,"source_line":2266,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":6132,"parent_record_offset":6100,"ancestor_record_offset":6012,"inlinee":6487,"source_line":32,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA3C
// 首条位置: {"module":3612,"inlinee":129596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3612,"record_offset":7132,"rva":41017904,"va":5409727024}
// 编译实例 RVAs: 0x271e230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6828228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409727024,"raw_hex":"e8c4306800","size":5,"soft_fail":false,"target_va":5416555360,"va":5409727127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-34048492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409727024,"raw_hex":"e81476f8fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5409727191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25108199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409727024,"raw_hex":"e819e180fe","size":5,"soft_fail":false,"target_va":5384618896,"va":5409727090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3612,"record_offset":7332,"parent_record_offset":7132,"ancestor_record_offset":7132,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3612,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":7132,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3612,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":7132,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3612,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":7132,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3612,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":7132,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3612,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":7132,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3612,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":7132,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3612,"record_offset":7356,"parent_record_offset":7332,"ancestor_record_offset":7132,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA43
// 首条位置: {"module":3613,"inlinee":129603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3613,"record_offset":31616,"parent_record_offset":31588,"ancestor_record_offset":31588,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3613,"record_offset":31640,"parent_record_offset":31616,"ancestor_record_offset":31588,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":31664,"parent_record_offset":31640,"ancestor_record_offset":31588,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":31692,"parent_record_offset":31664,"ancestor_record_offset":31588,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA38
// 首条位置: {"module":3612,"inlinee":129592}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA61
// 首条位置: {"module":3613,"inlinee":129633}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":31448,"parent_record_offset":31416,"ancestor_record_offset":31416,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":31840,"parent_record_offset":31816,"ancestor_record_offset":31416,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":31888,"parent_record_offset":31864,"ancestor_record_offset":31416,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":31864,"parent_record_offset":31840,"ancestor_record_offset":31416,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3613,"record_offset":31988,"parent_record_offset":31964,"ancestor_record_offset":31416,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3613,"record_offset":31964,"parent_record_offset":31448,"ancestor_record_offset":31416,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":31816,"parent_record_offset":31792,"ancestor_record_offset":31416,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":31792,"parent_record_offset":31768,"ancestor_record_offset":31416,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":31768,"parent_record_offset":31448,"ancestor_record_offset":31416,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31416,"inlinee":129603,"source_line":107,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":31532,"parent_record_offset":31508,"ancestor_record_offset":31416,"inlinee":129592,"source_line":107,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3613,"record_offset":31640,"parent_record_offset":31616,"ancestor_record_offset":31416,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":31664,"parent_record_offset":31640,"ancestor_record_offset":31416,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":31692,"parent_record_offset":31664,"ancestor_record_offset":31416,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3613,"record_offset":31616,"parent_record_offset":31588,"ancestor_record_offset":31416,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31740,"parent_record_offset":31448,"ancestor_record_offset":31416,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31564,"parent_record_offset":31448,"ancestor_record_offset":31416,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31508,"parent_record_offset":31448,"ancestor_record_offset":31416,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA5F
// 首条位置: {"module":3613,"inlinee":129631}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3613,"record_offset":32372,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3613,"record_offset":32576,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":32540,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":32920,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31184,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":32400,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3613,"record_offset":32768,"parent_record_offset":32740,"ancestor_record_offset":31108,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3613,"record_offset":32796,"parent_record_offset":32768,"ancestor_record_offset":31108,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3613,"record_offset":32832,"parent_record_offset":32796,"ancestor_record_offset":31108,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3613,"record_offset":32740,"parent_record_offset":32704,"ancestor_record_offset":31108,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":31840,"parent_record_offset":31816,"ancestor_record_offset":31108,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":32872,"parent_record_offset":32704,"ancestor_record_offset":31108,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":32704,"parent_record_offset":32672,"ancestor_record_offset":31108,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":31888,"parent_record_offset":31864,"ancestor_record_offset":31108,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":31864,"parent_record_offset":31840,"ancestor_record_offset":31108,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3613,"record_offset":32672,"parent_record_offset":32640,"ancestor_record_offset":31108,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3613,"record_offset":31988,"parent_record_offset":31964,"ancestor_record_offset":31108,"inlinee":4565,"source_line":1052,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":3613,"record_offset":31964,"parent_record_offset":31448,"ancestor_record_offset":31108,"inlinee":4563,"source_line":2822,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":31816,"parent_record_offset":31792,"ancestor_record_offset":31108,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":31792,"parent_record_offset":31768,"ancestor_record_offset":31108,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3613,"record_offset":31768,"parent_record_offset":31448,"ancestor_record_offset":31108,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31108,"inlinee":129603,"source_line":107,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":31532,"parent_record_offset":31508,"ancestor_record_offset":31108,"inlinee":129592,"source_line":107,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":31416,"parent_record_offset":31384,"ancestor_record_offset":31108,"inlinee":129633,"source_line":107,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":3613,"record_offset":31640,"parent_record_offset":31616,"ancestor_record_offset":31108,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3613,"record_offset":32608,"parent_record_offset":32576,"ancestor_record_offset":31108,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3613,"record_offset":31344,"parent_record_offset":31316,"ancestor_record_offset":31108,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":32324,"parent_record_offset":31248,"ancestor_record_offset":31108,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":31664,"parent_record_offset":31640,"ancestor_record_offset":31108,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3613,"record_offset":31692,"parent_record_offset":31664,"ancestor_record_offset":31108,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3613,"record_offset":31316,"parent_record_offset":31288,"ancestor_record_offset":31108,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3613,"record_offset":31616,"parent_record_offset":31588,"ancestor_record_offset":31108,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::MapRefDeserializer::next_pair
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":31288,"parent_record_offset":31248,"ancestor_record_offset":31108,"inlinee":11185,"source_line":2584,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":31448,"parent_record_offset":31416,"ancestor_record_offset":31108,"inlinee":17535,"source_line":2411,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":31248,"parent_record_offset":31216,"ancestor_record_offset":31108,"inlinee":11183,"source_line":2642,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$47::next_value_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":32448,"parent_record_offset":32424,"ancestor_record_offset":31108,"inlinee":11186,"source_line":2655,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31216,"parent_record_offset":31184,"ancestor_record_offset":31108,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":32424,"parent_record_offset":32400,"ancestor_record_offset":31108,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31740,"parent_record_offset":31448,"ancestor_record_offset":31108,"inlinee":10116,"source_line":1602,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31564,"parent_record_offset":31448,"ancestor_record_offset":31108,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31508,"parent_record_offset":31448,"ancestor_record_offset":31108,"inlinee":6493,"source_line":1422,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":31384,"parent_record_offset":31248,"ancestor_record_offset":31108,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":32496,"parent_record_offset":32472,"ancestor_record_offset":31108,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA62
// 首条位置: {"module":3613,"inlinee":129634}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":33820,"parent_record_offset":33540,"ancestor_record_offset":33540,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":33856,"parent_record_offset":33540,"ancestor_record_offset":33540,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":33572,"parent_record_offset":33540,"ancestor_record_offset":33540,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3613,"record_offset":33684,"parent_record_offset":33660,"ancestor_record_offset":33540,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":33772,"parent_record_offset":33628,"ancestor_record_offset":33540,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3613,"record_offset":33660,"parent_record_offset":33628,"ancestor_record_offset":33540,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":33628,"parent_record_offset":33600,"ancestor_record_offset":33540,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":33600,"parent_record_offset":33572,"ancestor_record_offset":33540,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":33716,"parent_record_offset":33628,"ancestor_record_offset":33540,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":33740,"parent_record_offset":33716,"ancestor_record_offset":33540,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$20::clone
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8187
// 首条位置: {"module":539,"inlinee":33159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$23::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8189
// 首条位置: {"module":539,"inlinee":33161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":539,"record_offset":41756,"parent_record_offset":41732,"ancestor_record_offset":41732,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":539,"record_offset":41780,"parent_record_offset":41756,"ancestor_record_offset":41732,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":539,"record_offset":41808,"parent_record_offset":41780,"ancestor_record_offset":41732,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":539,"record_offset":41836,"parent_record_offset":41808,"ancestor_record_offset":41732,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":539,"record_offset":41868,"parent_record_offset":41836,"ancestor_record_offset":41732,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$25::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14118
// 首条位置: {"module":1660,"inlinee":82200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1660,"record_offset":49560,"parent_record_offset":49536,"ancestor_record_offset":49536,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":49684,"parent_record_offset":49656,"ancestor_record_offset":49536,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1660,"record_offset":49584,"parent_record_offset":49560,"ancestor_record_offset":49536,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":49632,"parent_record_offset":49608,"ancestor_record_offset":49536,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1660,"record_offset":49656,"parent_record_offset":49632,"ancestor_record_offset":49536,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1660,"record_offset":49608,"parent_record_offset":49584,"ancestor_record_offset":49536,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/01660.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 113, 168, 174, 196, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA36
// 首条位置: {"module":3612,"inlinee":129590}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 113, 174, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA3E
// 首条位置: {"module":3612,"inlinee":129598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3612,"record_offset":7896,"rva":41018160,"va":5409727280}
// 编译实例 RVAs: 0x271e330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14271ef60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: config.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409727280,"raw_hex":"e8150c0000","size":5,"soft_fail":false,"target_va":5409730400,"va":5409727302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3612,"record_offset":8148,"parent_record_offset":7896,"ancestor_record_offset":7896,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3612,"record_offset":8200,"parent_record_offset":8148,"ancestor_record_offset":7896,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3612,"record_offset":8172,"parent_record_offset":8148,"ancestor_record_offset":7896,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA54
// 首条位置: {"module":3613,"inlinee":129620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":14700,"parent_record_offset":14676,"ancestor_record_offset":14676,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":14788,"parent_record_offset":14676,"ancestor_record_offset":14676,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":14752,"parent_record_offset":14700,"ancestor_record_offset":14676,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":14676,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA47
// 首条位置: {"module":3613,"inlinee":129607}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":15288,"parent_record_offset":15260,"ancestor_record_offset":15260,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":15260,"inlinee":98343,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":15344,"parent_record_offset":15288,"ancestor_record_offset":15260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3612,"record_offset":8244,"rva":41018352,"va":5409727472}
// 编译实例 RVAs: 0x271e3f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14271eb40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: config.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409727472,"raw_hex":"e819070000","size":5,"soft_fail":false,"target_va":5409729344,"va":5409727522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3612,"record_offset":8496,"parent_record_offset":8244,"ancestor_record_offset":8244,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3612,"record_offset":8520,"parent_record_offset":8496,"ancestor_record_offset":8244,"inlinee":98343,"source_line":232,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3612,"record_offset":8544,"parent_record_offset":8496,"ancestor_record_offset":8244,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 113, 174, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA55
// 首条位置: {"module":3613,"inlinee":129621}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3613,"record_offset":17544,"parent_record_offset":16420,"ancestor_record_offset":16420,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":17508,"parent_record_offset":16420,"ancestor_record_offset":16420,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":18832,"parent_record_offset":16420,"ancestor_record_offset":16420,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":16472,"parent_record_offset":16420,"ancestor_record_offset":16420,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":16420,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":16716,"parent_record_offset":16692,"ancestor_record_offset":16420,"inlinee":98343,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":16668,"parent_record_offset":16644,"ancestor_record_offset":16420,"inlinee":129623,"source_line":174,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":17008,"parent_record_offset":16976,"ancestor_record_offset":16420,"inlinee":129625,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$2::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":17760,"parent_record_offset":17728,"ancestor_record_offset":16420,"inlinee":129627,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$3::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":18224,"parent_record_offset":18192,"ancestor_record_offset":16420,"inlinee":129629,"source_line":113,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3613,"record_offset":16612,"parent_record_offset":16552,"ancestor_record_offset":16420,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3613,"record_offset":16584,"parent_record_offset":16552,"ancestor_record_offset":16420,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":17180,"parent_record_offset":16868,"ancestor_record_offset":16420,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":16744,"parent_record_offset":16692,"ancestor_record_offset":16420,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3613,"record_offset":16552,"parent_record_offset":16520,"ancestor_record_offset":16420,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3613,"record_offset":16520,"parent_record_offset":16496,"ancestor_record_offset":16420,"inlinee":15023,"source_line":2527,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":16496,"parent_record_offset":16472,"ancestor_record_offset":16420,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3613,"record_offset":16644,"parent_record_offset":16520,"ancestor_record_offset":16420,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":17068,"parent_record_offset":17040,"ancestor_record_offset":16420,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":25184,"parent_record_offset":25156,"ancestor_record_offset":24124,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 113, 174, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA57
// 首条位置: {"module":3613,"inlinee":129623}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":16668,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":29488,"parent_record_offset":29364,"ancestor_record_offset":29364,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3613,"record_offset":16716,"parent_record_offset":16692,"ancestor_record_offset":16668,"inlinee":98343,"source_line":196,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":16744,"parent_record_offset":16692,"ancestor_record_offset":16668,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 113, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA59
// 首条位置: {"module":3613,"inlinee":129625}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":17040,"parent_record_offset":17008,"ancestor_record_offset":17008,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":17136,"parent_record_offset":17008,"ancestor_record_offset":17008,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":17096,"parent_record_offset":17040,"ancestor_record_offset":17008,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":17068,"parent_record_offset":17040,"ancestor_record_offset":17008,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$2::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 113, 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA5B
// 首条位置: {"module":3613,"inlinee":129627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":17792,"parent_record_offset":17760,"ancestor_record_offset":17760,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":17888,"parent_record_offset":17760,"ancestor_record_offset":17760,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":17848,"parent_record_offset":17792,"ancestor_record_offset":17760,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":17820,"parent_record_offset":17792,"ancestor_record_offset":17760,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":25184,"parent_record_offset":25156,"ancestor_record_offset":25128,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$3::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA5D
// 首条位置: {"module":3613,"inlinee":129629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3613,"record_offset":18256,"parent_record_offset":18224,"ancestor_record_offset":18224,"inlinee":129594,"source_line":158,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":18352,"parent_record_offset":18224,"ancestor_record_offset":18224,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3613,"record_offset":18312,"parent_record_offset":18256,"ancestor_record_offset":18224,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$100::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3613,"record_offset":18284,"parent_record_offset":18256,"ancestor_record_offset":18224,"inlinee":7810,"source_line":1327,"archive":"_pdb/C/modules/03613.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$26::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC12
// 首条位置: {"module":3385,"inlinee":121874}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":5296,"parent_record_offset":5236,"ancestor_record_offset":5236,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":5236,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":5236,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":5236,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$6::inherit_from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18029
// 首条位置: {"module":2144,"inlinee":98345}
// 编译实例 RVAs: 0x2722f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3614,"record_offset":388,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3614,"record_offset":448,"parent_record_offset":420,"ancestor_record_offset":260,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3614,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":260,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3614,"record_offset":496,"parent_record_offset":472,"ancestor_record_offset":260,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3614,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":260,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03614.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA3B
// 首条位置: {"module":3612,"inlinee":129595}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$31::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC14
// 首条位置: {"module":3385,"inlinee":121876}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":5932,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":6028,"parent_record_offset":5980,"ancestor_record_offset":5896,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3385,"record_offset":5980,"parent_record_offset":5932,"ancestor_record_offset":5896,"inlinee":121878,"source_line":196,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":5896,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5896,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":5896,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$8::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1802C
// 首条位置: {"module":2144,"inlinee":98348}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$9::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1802E
// 首条位置: {"module":2144,"inlinee":98350}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$6::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":9068,"inlinee":95487,"source_line":152,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$11::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":3615,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":9068,"inlinee":98352,"source_line":223,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$34::clone
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DC16
// 首条位置: {"module":3385,"inlinee":121878}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3385,"record_offset":6028,"parent_record_offset":5980,"ancestor_record_offset":5980,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":5980,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3385,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":5980,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3385,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":5980,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$10::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA6B
// 首条位置: {"module":3615,"inlinee":129643}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":12408,"parent_record_offset":12364,"ancestor_record_offset":12364,"inlinee":98346,"source_line":30,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3615,"record_offset":12468,"parent_record_offset":12440,"ancestor_record_offset":12364,"inlinee":121867,"source_line":13,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3615,"record_offset":12496,"parent_record_offset":12468,"ancestor_record_offset":12364,"inlinee":8949,"source_line":446,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$23::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3615,"record_offset":12520,"parent_record_offset":12496,"ancestor_record_offset":12364,"inlinee":8951,"source_line":473,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3615,"record_offset":12544,"parent_record_offset":12520,"ancestor_record_offset":12364,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/03615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$11::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18030
// 首条位置: {"module":2144,"inlinee":98352}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3612,"record_offset":260,"rva":41014784,"va":5409723904}
// 编译实例 RVAs: 0x271d600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14271e690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: config.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723904,"raw_hex":"e87e100000","size":5,"soft_fail":false,"target_va":5409728144,"va":5409723917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e0e0b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: config.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7277047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409723904,"raw_hex":"e8f7096f00","size":5,"soft_fail":false,"target_va":5417001136,"va":5409724084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3612,"record_offset":528,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":3612,"record_offset":704,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::content::impl$52::unit_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3612,"record_offset":564,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6485,"source_line":2849,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":3612,"record_offset":500,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::content::impl$41::deserialize_unit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":3612,"record_offset":664,"parent_record_offset":636,"ancestor_record_offset":260,"inlinee":6489,"source_line":2266,"archive":"_pdb/C/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":3612,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":260,"inlinee":6487,"source_line":32,"archive":"_pdb/C/modules/03612.jsonl.gz"}
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
// 完整结构及原始类型记录: config.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::AudioTriggerConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>", "constructor": "asset::inheritable::impl$1::inherit_from", "arguments": ["asset::prefab::config::AudioTriggerConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::MeshProcessorConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "enum2$<asset::prefab::config::MeshProcessorConfig>", "constructor": "enum2$", "arguments": ["asset::prefab::config::MeshProcessorConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> >", "constructor": "core::option::Option", "arguments": ["enum2$<asset::prefab::config::MeshProcessorConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "constructor": "tuple$", "arguments": ["alloc::string::String", "enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "std::collections::hash::map::VacantEntry", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > >", "constructor": "ref$", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > > >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::Deserializer>", "enum2$<asset::prefab::config::MeshProcessorConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::tag::Deserializer>", "enum2$<asset::prefab::config::MeshProcessorConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::PrefabAudioConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabAudioConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabAudioConfig>", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::Deserializer>", "asset::prefab::config::PrefabAudioConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<asset::prefab::config::PrefabAudioConfig>", "constructor": "core::option::Option", "arguments": ["asset::prefab::config::PrefabAudioConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> >", "constructor": "enum2$", "arguments": ["core::option::Option<asset::prefab::config::PrefabAudioConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >", "constructor": "asset::inheritable::impl$1::inherit_from", "arguments": ["enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::PrefabConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "ref$<asset::prefab::config::PrefabConfig>", "constructor": "ref$", "arguments": ["asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> >", "constructor": "tuple$", "arguments": ["ref$<asset::prefab::PrefabName>", "ref$<asset::prefab::config::PrefabConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > >", "constructor": "ref$", "arguments": ["tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >", "constructor": "tuple$", "arguments": ["ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >", "country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >", "country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "constructor": "tuple$", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "constructor": "std::collections::hash::map::Iter", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>", "country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<asset::prefab::PrefabName,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["asset::prefab::PrefabName", "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<std::collections::hash::map::Iter<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$0>,country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<asset::prefab::config::PrefabConfig>", "constructor": "core::marker::PhantomData", "arguments": ["asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_str_seed<core::marker::PhantomData<asset::prefab::config::PrefabConfig>,asset::prefab::config::PrefabConfig>", "constructor": "ron::options::Options::from_str_seed", "arguments": ["core::marker::PhantomData<asset::prefab::config::PrefabConfig>", "asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::PrefabPartConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::prefab::config::PrefabPartConfig>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::prefab::config::PrefabPartConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>", "constructor": "tuple$", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>", "constructor": "asset::inheritable::impl$5::inherit_from", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>", "constructor": "serde_core::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde_core::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::PrefabRenderConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<asset::prefab::config::PrefabRenderConfig>", "constructor": "core::ptr::drop_in_place", "arguments": ["asset::prefab::config::PrefabRenderConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>", "constructor": "asset::inheritable::impl$1::inherit_from", "arguments": ["asset::prefab::config::PrefabRenderConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::PrefabRtConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabRtConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::Deserializer>,asset::prefab::config::PrefabRtConfig>", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::Deserializer>", "asset::prefab::config::PrefabRtConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<asset::prefab::config::PrefabRtConfig>", "constructor": "core::option::Option", "arguments": ["asset::prefab::config::PrefabRtConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> >", "constructor": "enum2$", "arguments": ["core::option::Option<asset::prefab::config::PrefabRtConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >", "constructor": "asset::inheritable::impl$1::inherit_from", "arguments": ["enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: config.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: config.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)
// 候选参数类型: ["ref$<country_core::resources::clutter_packs::ClutterPackObject>"]

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize
// 信息 ID: bc53a2a437bf83685c947448
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<asset::prefab::config::PrefabConfig>,asset::prefab::config::PrefabConfig>; module=3383, inline_record=548; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >; module=3612, inline_record=8520; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=14472; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=15316; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=16716; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=22824; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=24436; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=28400; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 38669e504e88b846cce0e00f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=3612, inline_record=8756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 信息 ID: 4658a7f369f75acd975b52d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_bytes -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: e336735a013dc252fb686b6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=660; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=944; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=3613, inline_record=1884; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=3613, inline_record=2392; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=3180; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=3464; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=4356; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=4640; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: fff528d1aeeeab1bf9cfcfee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x271f6a8 -> 0x271db70 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_str<asset::prefab::config::_::impl$0::deserialize::__FieldVisitor> @ 0x2a7ca28 -> 0x271ddd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_str<asset::prefab::config::_::impl$0::deserialize::__FieldVisitor> @ 0x2a7ca72 -> 0x271ddd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2a7eef9 -> 0x271ddd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2a7f1a4 -> 0x271ddd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: 8d572b43b8ea112b0d760a0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=3613, inline_record=1832; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=3613, inline_record=2168; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=13720; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=13952; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=20872; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=21552; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=27316; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=27720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 741607e2ec60f817afb46564
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=552; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=3613, inline_record=1716; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=3072; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$51::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<enum2$<asset::prefab::config::_::impl$0::deserialize::__Field> > >; module=3613, inline_record=4248; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=13552; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=20728; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=27200; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=29896; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: dd9a4b44a80a238556893f1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3612, inline_record=9180; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3612, inline_record=9380; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3612, inline_record=9580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 635eab0e150aa9f0c4dbd6f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=6012; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=6740; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=7296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::expecting
// 信息 ID: b1bd51c0d7832a59abdef7bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=3612, inline_record=8980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 信息 ID: f61d44d72ed23503f5d2d9df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: 4d757c52bb2190aef3137166
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=3613, inline_record=31588; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=3613, inline_record=32160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 信息 ID: f98fbba59935e8466ac93f28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=3613, inline_record=31532; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=3613, inline_record=31936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: ac1c6eae1496a1dfa2aee4ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=3613, inline_record=31416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: b0c7e92f8d078608c1bb11ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=3613, inline_record=31108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: 78ac8255999cac12e10a2ceb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$52::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=3613, inline_record=33540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum<serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 12a63e630990748553071ff1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::EnumRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x271eb73 -> 0x271d600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 430e1819a8658c792863374c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=13132; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=20384; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=26884; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=29440; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=40528; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=49232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: 240f067a2d2df901279e6ff6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=14424; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=22768; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=28328; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=31348; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=32216; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=42072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 信息 ID: e56216d76e1b5f34c7953a1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=15544; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=22396; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=34128; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=34244; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=42776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
// 信息 ID: ca9fa2b5190876df9ee3eec9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=15852; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=23052; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=35512; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=36420; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=44468; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=44628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 785b311b807e51b62e53870e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::ContentRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize -> ["serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x271fc57 -> 0x271e330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
// 信息 ID: afd91ee12248599909998488
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=14960; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=45208; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4501, inline_record=45424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$4::deserialize
// 信息 ID: 5f095ecf69ca31cd87c45745
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=14676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize
// 信息 ID: 400455af5096146eedd7fb87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=15260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize<serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: 6ec07a32befe0dc79a1bd667
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::private::de::content::ContentRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize -> ["serde::private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x271fcc8 -> 0x271e3f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: a0fe3d0e4f44e44a43c3e617
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=16420; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=24124; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=29152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 信息 ID: 9d0d78158de1b41671f548b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=16668; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=24380; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=29364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 信息 ID: c267447d48b94ee211405f62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=17008; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=24728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$2::deserialize
// 信息 ID: debf6369856070d4de7bdeb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=17760; 不是独立调用边。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=25128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$3::deserialize
// 信息 ID: 55abdba7c4d9d7c9e5b92dba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::private::de::content::impl$41::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=3613, inline_record=18224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$10::inherit_from
// 信息 ID: d6efa57de4c092036ecbdda9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>; module=3615, inline_record=12364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$11::concrete_default
// 信息 ID: 1807b7e07b8b7150ef183e3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=3615, inline_record=9116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$13::default
// 信息 ID: 06aaa693f4448d670c479214
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree; module=1805, inline_record=2008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$14::clone
// 信息 ID: fbec7e7e1ac1cb9ec75d889b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$17::clone
// 信息 ID: 34766470a663727f1cadcfc3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4316; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>; module=3615, inline_record=10232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$1::inherit_from
// 信息 ID: fe3e90a6adcaaf6b51519f70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=1805, inline_record=13588; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=1805, inline_record=19896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$20::clone
// 信息 ID: 06e415c5daec042e62ac0a18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=40520; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=40584; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=4460; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::impl$5::inherit_from; module=3614, inline_record=1384; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>; module=3615, inline_record=10380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$23::eq
// 信息 ID: 8008de1ce92ae9da99a43181
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=41732; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>; module=1658, inline_record=54420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$25::hash
// 信息 ID: 3f81458269bbaa577208c9cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > > >; module=1660, inline_record=49536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$26::clone
// 信息 ID: 9f3e011eceb430e231877846
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=5236; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >; module=3615, inline_record=13920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$2::concrete_default
// 信息 ID: 30dbe93980974fd3c0e25918
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=2236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$31::clone
// 信息 ID: 0d53af1fc9a55cfe0b5bfb60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=5896; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=3615, inline_record=9248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$34::clone
// 信息 ID: b5a90e6c4a134395b7178314
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=5980; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=3615, inline_record=9316; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>; module=3615, inline_record=12712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$3::inherit_from
// 信息 ID: 5d29511522195890bc44cd41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=2264; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=3385, inline_record=3924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$4::concrete_default
// 信息 ID: 5a55d2444ca6546ce1953e2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$5::inherit_from
// 信息 ID: 7974c5d4b3fde1f0a2ea61af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig> @ 0x272565f -> 0x27230d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$6::inherit_from
// 信息 ID: e0af73a8bc45023a4f2e8e11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > > @ 0x2725d0c -> 0x2722f60 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >; module=3615, inline_record=13572; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$8::inherit_from
// 信息 ID: 4692fe847599a0b7e1b100b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=3615, inline_record=9040; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=3615, inline_record=9152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$9::concrete_default
// 信息 ID: 310f7d4c36ac51546e959645
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=3615, inline_record=9068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
