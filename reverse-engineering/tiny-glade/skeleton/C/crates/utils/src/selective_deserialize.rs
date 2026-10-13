// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/selective_deserialize.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d242952ca071cd6f17c66c0c87a716bc158cfc9d72e1cc2a91b3b5d84466a1d2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: selective_deserialize.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 533, 1645
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E80
// 首条位置: {"module":533,"inlinee":32384}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$0::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E82
// 首条位置: {"module":1645,"inlinee":81538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":422084,"parent_record_offset":421144,"ancestor_record_offset":421144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1645,"record_offset":422052,"parent_record_offset":421144,"ancestor_record_offset":421144,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":421192,"parent_record_offset":421144,"ancestor_record_offset":421144,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":421952,"parent_record_offset":421144,"ancestor_record_offset":421144,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":422320,"parent_record_offset":422296,"ancestor_record_offset":421144,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":422344,"parent_record_offset":422320,"ancestor_record_offset":421144,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":422368,"parent_record_offset":422344,"ancestor_record_offset":421144,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":422296,"parent_record_offset":422204,"ancestor_record_offset":421144,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":421596,"parent_record_offset":421572,"ancestor_record_offset":421144,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":422232,"parent_record_offset":422204,"ancestor_record_offset":421144,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":422204,"parent_record_offset":422180,"ancestor_record_offset":421144,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":421644,"parent_record_offset":421620,"ancestor_record_offset":421144,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":421620,"parent_record_offset":421596,"ancestor_record_offset":421144,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":422180,"parent_record_offset":422156,"ancestor_record_offset":421144,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":421572,"parent_record_offset":421536,"ancestor_record_offset":421144,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":421728,"parent_record_offset":421352,"ancestor_record_offset":421144,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":421512,"parent_record_offset":421420,"ancestor_record_offset":421144,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":421536,"parent_record_offset":421512,"ancestor_record_offset":421144,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":421780,"parent_record_offset":421756,"ancestor_record_offset":421144,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1645,"record_offset":421476,"parent_record_offset":421448,"ancestor_record_offset":421144,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1645,"record_offset":422264,"parent_record_offset":422232,"ancestor_record_offset":421144,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1645,"record_offset":421448,"parent_record_offset":421420,"ancestor_record_offset":421144,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":422108,"parent_record_offset":422084,"ancestor_record_offset":421144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":421684,"parent_record_offset":421536,"ancestor_record_offset":421144,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":421852,"parent_record_offset":421828,"ancestor_record_offset":421144,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":421880,"parent_record_offset":421852,"ancestor_record_offset":421144,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":421804,"parent_record_offset":421780,"ancestor_record_offset":421144,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":421280,"parent_record_offset":421224,"ancestor_record_offset":421144,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":436516,"parent_record_offset":436492,"ancestor_record_offset":435620,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":421396,"parent_record_offset":421352,"ancestor_record_offset":421144,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":421224,"parent_record_offset":421192,"ancestor_record_offset":421144,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":421976,"parent_record_offset":421952,"ancestor_record_offset":421144,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":421352,"parent_record_offset":421328,"ancestor_record_offset":421144,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":421328,"parent_record_offset":421304,"ancestor_record_offset":421144,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$4::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":421420,"parent_record_offset":421396,"ancestor_record_offset":421144,"inlinee":34220,"source_line":307,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] utils::selective_deserialize::impl$4::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/selective_deserialize.rs
//     evidence={"module":1645,"record_offset":421756,"parent_record_offset":421352,"ancestor_record_offset":421144,"inlinee":32438,"source_line":139,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] utils::selective_deserialize::impl$5::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/selective_deserialize.rs
//     evidence={"module":1645,"record_offset":421304,"parent_record_offset":421280,"ancestor_record_offset":421144,"inlinee":81540,"source_line":156,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$1::deserialize<alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>,country_core::systems::save::snapshot_gc::SnapshotsKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":2268,"rva":10639680,"va":5379348800}
// 编译实例 RVAs: 0xa25940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3670237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379348800,"raw_hex":"e823ffc7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379348936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10010107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379348800,"raw_hex":"e8054267ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379348950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141677ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12919115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379348800,"raw_hex":"e84b21c500","size":5,"soft_fail":false,"target_va":5392268000,"va":5379348880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":2684,"parent_record_offset":2268,"ancestor_record_offset":2268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":2268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":2268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":2268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":2268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":2268,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":2268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":3340,"parent_record_offset":3316,"ancestor_record_offset":2268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":3316,"parent_record_offset":3292,"ancestor_record_offset":2268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":533,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":2268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$1::deserialize<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >,country_core::systems::save::snapshot_gc::HistoryKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":533,"record_offset":844,"rva":10639328,"va":5379348448}
// 编译实例 RVAs: 0xa257e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3669885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379348448,"raw_hex":"e88300c8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379348584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10009755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379348448,"raw_hex":"e8654367ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379348598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141676430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: selective_deserialize.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12913659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379348448,"raw_hex":"e8fb0bc500","size":5,"soft_fail":false,"target_va":5392262192,"va":5379348528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":1368,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":844,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":844,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":844,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":1532,"parent_record_offset":1504,"ancestor_record_offset":844,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":533,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":844,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":2076,"parent_record_offset":2048,"ancestor_record_offset":844,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":844,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":844,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":533,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":844,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":1392,"parent_record_offset":1368,"ancestor_record_offset":844,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$4::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EB6
// 首条位置: {"module":1645,"inlinee":32438}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":421780,"parent_record_offset":421756,"ancestor_record_offset":421756,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":421828,"parent_record_offset":421804,"ancestor_record_offset":421756,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":421852,"parent_record_offset":421828,"ancestor_record_offset":421756,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":421880,"parent_record_offset":421852,"ancestor_record_offset":421756,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":421804,"parent_record_offset":421780,"ancestor_record_offset":421756,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::selective_deserialize::impl$5::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E84
// 首条位置: {"module":1645,"inlinee":81540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":421328,"parent_record_offset":421304,"ancestor_record_offset":421304,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":421596,"parent_record_offset":421572,"ancestor_record_offset":421304,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":421644,"parent_record_offset":421620,"ancestor_record_offset":421304,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":421620,"parent_record_offset":421596,"ancestor_record_offset":421304,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":421572,"parent_record_offset":421536,"ancestor_record_offset":421304,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":421728,"parent_record_offset":421352,"ancestor_record_offset":421304,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":421512,"parent_record_offset":421420,"ancestor_record_offset":421304,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":421536,"parent_record_offset":421512,"ancestor_record_offset":421304,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1645,"record_offset":421780,"parent_record_offset":421756,"ancestor_record_offset":421304,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1645,"record_offset":421476,"parent_record_offset":421448,"ancestor_record_offset":421304,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1645,"record_offset":421448,"parent_record_offset":421420,"ancestor_record_offset":421304,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":421684,"parent_record_offset":421536,"ancestor_record_offset":421304,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":421852,"parent_record_offset":421828,"ancestor_record_offset":421304,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1645,"record_offset":421880,"parent_record_offset":421852,"ancestor_record_offset":421304,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1645,"record_offset":421804,"parent_record_offset":421780,"ancestor_record_offset":421304,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":421396,"parent_record_offset":421352,"ancestor_record_offset":421304,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":421352,"parent_record_offset":421328,"ancestor_record_offset":421304,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$4::discard
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":421420,"parent_record_offset":421396,"ancestor_record_offset":421304,"inlinee":34220,"source_line":307,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] utils::selective_deserialize::impl$4::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/selective_deserialize.rs
//     evidence={"module":1645,"record_offset":421756,"parent_record_offset":421352,"ancestor_record_offset":421304,"inlinee":32438,"source_line":139,"archive":"_pdb/C/modules/01645.jsonl.gz"}
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
// 完整结构及原始类型记录: selective_deserialize.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::selective_deserialize::PartialVisitor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$1::fmt<utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde_core::de::impl$1::fmt", "arguments": ["utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::selective_deserialize::SelectiveDeserialize
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "constructor": "utils::selective_deserialize::SelectiveDeserialize", "arguments": ["country_core::systems::save::snapshot_gc::SnapshotsKey", "alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::impl$1::deserialize<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >,country_core::systems::save::snapshot_gc::HistoryKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >", "constructor": "utils::selective_deserialize::impl$1::deserialize", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "country_core::systems::save::snapshot_gc::HistoryKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impl$1::fmt<utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde_core::de::impl$1::fmt", "arguments": ["utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >", "constructor": "utils::selective_deserialize::SelectiveDeserialize", "arguments": ["country_core::systems::save::snapshot_gc::HistoryKey", "utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "utils::selective_deserialize::PartialVisitor", "arguments": ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >", "constructor": "serde_json::de::impl$7::deserialize_struct", "arguments": ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >", "utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: selective_deserialize.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: selective_deserialize.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::prefab::PrefabsModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::prefab::PrefabsModifiedCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::prefab::PrefabsModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::grass2::Grass2RuntimeConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$0::expecting
// 信息 ID: f76ff1e9f6edab2e9cb05f66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >; module=533, inline_record=149180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$0::visit_map
// 信息 ID: fda392e87c600519e571a0f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >; module=1645, inline_record=421144; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >; module=1645, inline_record=435620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$1::deserialize<alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>,country_core::systems::save::snapshot_gc::SnapshotsKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >
// 信息 ID: f52a1121e45f9335d84735f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["country_core::resources::history::ResolvedHistoryChainSnapshot", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] serde_json::read::IoRead -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > >"]; 字段布局 = unknown。
// [类型关系] utils::selective_deserialize::impl$1::deserialize -> ["alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>", "country_core::systems::save::snapshot_gc::SnapshotsKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > > @ 0x16766a0 -> 0xa25940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$1::deserialize<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >,country_core::systems::save::snapshot_gc::HistoryKey,ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > > >
// 信息 ID: 21def19d6ce147309748498e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["country_core::resources::history::ResolvedHistoryChainSnapshot", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] utils::selective_deserialize::SelectiveDeserialize -> ["country_core::systems::save::snapshot_gc::SnapshotsKey", "alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] serde_json::read::IoRead -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// [类型关系] serde_json::de::Deserializer -> ["serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > >"]; 字段布局 = unknown。
// [类型关系] utils::selective_deserialize::impl$1::deserialize -> ["utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> >", "country_core::systems::save::snapshot_gc::HistoryKey", "ref_mut$<serde_json::de::Deserializer<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot_gc::parse_save_file_snapshots<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x1c14a58 -> 0xa257e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$4::visit_str
// 信息 ID: cbb219338b487cbc4a7d66d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >; module=1645, inline_record=421756; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >; module=1645, inline_record=436232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::selective_deserialize::impl$5::deserialize
// 信息 ID: 231d62b7a0a6fd38bf6b2ce8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::HistoryKey,utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > > >; module=1645, inline_record=421304; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::IoRead<std::io::buffered::bufreader::BufReader<std::fs::File> >,utils::selective_deserialize::PartialVisitor<utils::selective_deserialize::SelectiveDeserialize<country_core::systems::save::snapshot_gc::SnapshotsKey,alloc::vec::Vec<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global> > > >; module=1645, inline_record=435780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
