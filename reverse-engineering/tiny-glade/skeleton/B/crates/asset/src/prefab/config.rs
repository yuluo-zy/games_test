// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset/src/prefab/config.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: bda6aee0cbce1e10856e25fcbede434542f352ddcccbff51e83e6b047e106843
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: config.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 575, 1553, 1582, 1613, 3085, 4091, 4094, 4583, 4589, 4590, 4592, 4605, 4742, 4747, 4753
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
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=50
// 源码行: 6, 27, 71, 108, 114, 169, 175, 197, 233, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E857
// 首条位置: {"module":4094,"inlinee":125015}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":17204,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 6, 71, 175, 197, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BC2
// 首条位置: {"module":4589,"inlinee":134082}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4589,"record_offset":40088,"parent_record_offset":40060,"ancestor_record_offset":40060,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":40112,"parent_record_offset":40088,"ancestor_record_offset":40060,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":40136,"parent_record_offset":40112,"ancestor_record_offset":40060,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":40160,"parent_record_offset":40136,"ancestor_record_offset":40060,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=32
// 源码行: 6, 27, 71, 108, 114, 169, 175, 197, 233, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BC0
// 首条位置: {"module":4589,"inlinee":134080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39952,"parent_record_offset":39920,"ancestor_record_offset":39920,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":548,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":548,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":548,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":548,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4605,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":548,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4605,"record_offset":636,"parent_record_offset":580,"ancestor_record_offset":548,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":548,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":548,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":832,"parent_record_offset":580,"ancestor_record_offset":548,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":40060,"parent_record_offset":40032,"ancestor_record_offset":39920,"inlinee":134082,"source_line":71,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":1372,"inlinee":134112,"source_line":175,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":40112,"parent_record_offset":40088,"ancestor_record_offset":39920,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":39996,"parent_record_offset":39952,"ancestor_record_offset":39920,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":40212,"parent_record_offset":39952,"ancestor_record_offset":39920,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":40136,"parent_record_offset":40112,"ancestor_record_offset":39920,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":40160,"parent_record_offset":40136,"ancestor_record_offset":39920,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4589,"record_offset":40088,"parent_record_offset":40060,"ancestor_record_offset":39920,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":40032,"parent_record_offset":39952,"ancestor_record_offset":39920,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":1456,"parent_record_offset":1400,"ancestor_record_offset":1372,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 6, 27, 71, 114, 175, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BBE
// 首条位置: {"module":4589,"inlinee":134078}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4589,"record_offset":40364,"parent_record_offset":39260,"ancestor_record_offset":39260,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":41072,"parent_record_offset":39260,"ancestor_record_offset":39260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":40328,"parent_record_offset":39260,"ancestor_record_offset":39260,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39320,"parent_record_offset":39260,"ancestor_record_offset":39260,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":40392,"parent_record_offset":39260,"ancestor_record_offset":39260,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":14252,"parent_record_offset":14204,"ancestor_record_offset":14204,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":14620,"parent_record_offset":14204,"ancestor_record_offset":14204,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4589,"record_offset":41212,"parent_record_offset":39260,"ancestor_record_offset":39260,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44952,"parent_record_offset":44928,"ancestor_record_offset":42532,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44976,"parent_record_offset":44952,"ancestor_record_offset":42532,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":45400,"parent_record_offset":45376,"ancestor_record_offset":42532,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44928,"parent_record_offset":44752,"ancestor_record_offset":42532,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":45376,"parent_record_offset":45352,"ancestor_record_offset":42532,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44692,"parent_record_offset":44664,"ancestor_record_offset":42532,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":54440,"parent_record_offset":54412,"ancestor_record_offset":50920,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":45352,"parent_record_offset":45328,"ancestor_record_offset":42532,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44752,"parent_record_offset":44720,"ancestor_record_offset":42532,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44720,"parent_record_offset":44692,"ancestor_record_offset":42532,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":45328,"parent_record_offset":45304,"ancestor_record_offset":42532,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44788,"parent_record_offset":44752,"ancestor_record_offset":42532,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44624,"parent_record_offset":44588,"ancestor_record_offset":42532,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44588,"parent_record_offset":44552,"ancestor_record_offset":42532,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44516,"parent_record_offset":44480,"ancestor_record_offset":42532,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44552,"parent_record_offset":44516,"ancestor_record_offset":42532,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44480,"parent_record_offset":44444,"ancestor_record_offset":42532,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4589,"record_offset":44444,"parent_record_offset":44408,"ancestor_record_offset":42532,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4589,"record_offset":44664,"parent_record_offset":44624,"ancestor_record_offset":42532,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=14
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":44156,"parent_record_offset":44124,"ancestor_record_offset":42532,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":54024,"parent_record_offset":54000,"ancestor_record_offset":50920,"inlinee":125015,"source_line":71,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":40060,"parent_record_offset":40032,"ancestor_record_offset":39260,"inlinee":134082,"source_line":71,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":39640,"parent_record_offset":39616,"ancestor_record_offset":39260,"inlinee":134080,"source_line":27,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":44124,"parent_record_offset":44088,"ancestor_record_offset":42532,"inlinee":134084,"source_line":27,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":47908,"parent_record_offset":47884,"ancestor_record_offset":42532,"inlinee":134088,"source_line":27,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":46012,"parent_record_offset":45976,"ancestor_record_offset":42532,"inlinee":134086,"source_line":27,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":52704,"parent_record_offset":52676,"ancestor_record_offset":50920,"inlinee":134090,"source_line":27,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$4::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":18736,"parent_record_offset":18712,"ancestor_record_offset":17620,"inlinee":134187,"source_line":114,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":19560,"parent_record_offset":19536,"ancestor_record_offset":17620,"inlinee":134119,"source_line":114,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":44812,"parent_record_offset":44788,"ancestor_record_offset":42532,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":44836,"parent_record_offset":44812,"ancestor_record_offset":42532,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":40112,"parent_record_offset":40088,"ancestor_record_offset":39260,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":45052,"parent_record_offset":45028,"ancestor_record_offset":42532,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":44884,"parent_record_offset":44860,"ancestor_record_offset":42532,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4605,"record_offset":14436,"parent_record_offset":14404,"ancestor_record_offset":14204,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4605,"record_offset":14404,"parent_record_offset":14368,"ancestor_record_offset":14204,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":42532,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":39544,"parent_record_offset":39356,"ancestor_record_offset":39260,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":39672,"parent_record_offset":39356,"ancestor_record_offset":39260,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4605,"record_offset":14468,"parent_record_offset":14436,"ancestor_record_offset":14204,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4589,"record_offset":45028,"parent_record_offset":44624,"ancestor_record_offset":42532,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=160
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":40840,"parent_record_offset":40436,"ancestor_record_offset":39260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":40252,"parent_record_offset":39356,"ancestor_record_offset":39260,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=56
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":39508,"parent_record_offset":39356,"ancestor_record_offset":39260,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":40212,"parent_record_offset":39952,"ancestor_record_offset":39260,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":40136,"parent_record_offset":40112,"ancestor_record_offset":39260,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":40160,"parent_record_offset":40136,"ancestor_record_offset":39260,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4589,"record_offset":40088,"parent_record_offset":40060,"ancestor_record_offset":39260,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4589,"record_offset":40912,"parent_record_offset":40888,"ancestor_record_offset":39260,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4589,"record_offset":39576,"parent_record_offset":39356,"ancestor_record_offset":39260,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_bool
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":52184,"parent_record_offset":52160,"ancestor_record_offset":50920,"inlinee":6888,"source_line":345,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39952,"parent_record_offset":39920,"ancestor_record_offset":39260,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":44300,"parent_record_offset":44264,"ancestor_record_offset":42532,"inlinee":5163,"source_line":445,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":44264,"parent_record_offset":44228,"ancestor_record_offset":42532,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39356,"parent_record_offset":39320,"ancestor_record_offset":39260,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":40436,"parent_record_offset":40392,"ancestor_record_offset":39260,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::Deserializer::new
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":47856,"parent_record_offset":47684,"ancestor_record_offset":42532,"inlinee":100071,"source_line":10,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_bool
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":52160,"parent_record_offset":52136,"ancestor_record_offset":50920,"inlinee":6908,"source_line":39,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":40672,"parent_record_offset":40648,"ancestor_record_offset":39260,"inlinee":100077,"source_line":214,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":44228,"parent_record_offset":44192,"ancestor_record_offset":42532,"inlinee":5160,"source_line":139,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":54048,"parent_record_offset":54024,"ancestor_record_offset":50920,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":14288,"parent_record_offset":14252,"ancestor_record_offset":14204,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":14648,"parent_record_offset":14620,"ancestor_record_offset":14204,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":40032,"parent_record_offset":39952,"ancestor_record_offset":39260,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":39616,"parent_record_offset":39356,"ancestor_record_offset":39260,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":40648,"parent_record_offset":40624,"ancestor_record_offset":39260,"inlinee":73646,"source_line":1505,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":52136,"parent_record_offset":52112,"ancestor_record_offset":50920,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":44408,"parent_record_offset":44372,"ancestor_record_offset":42532,"inlinee":6423,"source_line":603,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":44192,"parent_record_offset":44156,"ancestor_record_offset":42532,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":18276,"parent_record_offset":18212,"ancestor_record_offset":17620,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::MapDeserializer::next_pair
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":4605,"record_offset":14368,"parent_record_offset":14324,"ancestor_record_offset":14204,"inlinee":6979,"source_line":1146,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_key_seed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":4605,"record_offset":14324,"parent_record_offset":14288,"ancestor_record_offset":14204,"inlinee":7004,"source_line":1210,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$57::next_value_seed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":4605,"record_offset":14676,"parent_record_offset":14648,"ancestor_record_offset":14204,"inlinee":7006,"source_line":1223,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":4589,"record_offset":41260,"parent_record_offset":41236,"ancestor_record_offset":39260,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":4589,"record_offset":41236,"parent_record_offset":41212,"ancestor_record_offset":39260,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$13::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F56
// 首条位置: {"module":3085,"inlinee":106326}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3085,"record_offset":4732,"parent_record_offset":4704,"ancestor_record_offset":4704,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3085,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":4704,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3085,"record_offset":5320,"parent_record_offset":5288,"ancestor_record_offset":4704,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3085,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":4704,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3085,"record_offset":5288,"parent_record_offset":5228,"ancestor_record_offset":4704,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3085,"record_offset":4864,"parent_record_offset":4828,"ancestor_record_offset":4704,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3085,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":4704,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3085,"record_offset":5068,"parent_record_offset":4952,"ancestor_record_offset":4704,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":5144,"parent_record_offset":5116,"ancestor_record_offset":4704,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":4704,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3085,"record_offset":5172,"parent_record_offset":5116,"ancestor_record_offset":4704,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":4704,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3085,"record_offset":5028,"parent_record_offset":5004,"ancestor_record_offset":4704,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3085,"record_offset":4892,"parent_record_offset":4864,"ancestor_record_offset":4704,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":5228,"parent_record_offset":4828,"ancestor_record_offset":4704,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3085,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":4704,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3085,"record_offset":4920,"parent_record_offset":4892,"ancestor_record_offset":4704,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3085,"record_offset":4952,"parent_record_offset":4920,"ancestor_record_offset":4704,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3085,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":4704,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3085,"record_offset":4804,"parent_record_offset":4780,"ancestor_record_offset":4704,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03085.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$1::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19F58
// 首条位置: {"module":3085,"inlinee":106328}
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
// 源码行: 27, 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4592,"record_offset":1460,"rva":40418928,"va":5409128048}
// 编译实例 RVAs: 0x268be70, 0x268c120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2353122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409128048,"raw_hex":"e8e2e72300","size":5,"soft_fail":false,"target_va":5411481328,"va":5409128201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4592,"record_offset":1636,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4592,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":1460,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4592,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":1460,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4592,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":1460,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 27, 71, 114, 175, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BC4
// 首条位置: {"module":4589,"inlinee":134084}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":44156,"parent_record_offset":44124,"ancestor_record_offset":44124,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":14860,"parent_record_offset":14732,"ancestor_record_offset":14732,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44952,"parent_record_offset":44928,"ancestor_record_offset":44124,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44976,"parent_record_offset":44952,"ancestor_record_offset":44124,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":44928,"parent_record_offset":44752,"ancestor_record_offset":44124,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44692,"parent_record_offset":44664,"ancestor_record_offset":44124,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44752,"parent_record_offset":44720,"ancestor_record_offset":44124,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44720,"parent_record_offset":44692,"ancestor_record_offset":44124,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":44788,"parent_record_offset":44752,"ancestor_record_offset":44124,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44624,"parent_record_offset":44588,"ancestor_record_offset":44124,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44588,"parent_record_offset":44552,"ancestor_record_offset":44124,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44516,"parent_record_offset":44480,"ancestor_record_offset":44124,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44552,"parent_record_offset":44516,"ancestor_record_offset":44124,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":44480,"parent_record_offset":44444,"ancestor_record_offset":44124,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4589,"record_offset":44444,"parent_record_offset":44408,"ancestor_record_offset":44124,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4589,"record_offset":44664,"parent_record_offset":44624,"ancestor_record_offset":44124,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":14788,"parent_record_offset":14760,"ancestor_record_offset":14732,"inlinee":125015,"source_line":233,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":44812,"parent_record_offset":44788,"ancestor_record_offset":44124,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":44836,"parent_record_offset":44812,"ancestor_record_offset":44124,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":45052,"parent_record_offset":45028,"ancestor_record_offset":44124,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":44884,"parent_record_offset":44860,"ancestor_record_offset":44124,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":44860,"parent_record_offset":44836,"ancestor_record_offset":44124,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4589,"record_offset":45028,"parent_record_offset":44624,"ancestor_record_offset":44124,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":44340,"parent_record_offset":44300,"ancestor_record_offset":44124,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_bool
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":52184,"parent_record_offset":52160,"ancestor_record_offset":52088,"inlinee":6888,"source_line":345,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":44300,"parent_record_offset":44264,"ancestor_record_offset":44124,"inlinee":5163,"source_line":445,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":44264,"parent_record_offset":44228,"ancestor_record_offset":44124,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_bool
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":52160,"parent_record_offset":52136,"ancestor_record_offset":52088,"inlinee":6908,"source_line":39,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":44228,"parent_record_offset":44192,"ancestor_record_offset":44124,"inlinee":5160,"source_line":139,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":44372,"parent_record_offset":44300,"ancestor_record_offset":44124,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":52136,"parent_record_offset":52112,"ancestor_record_offset":52088,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":44408,"parent_record_offset":44372,"ancestor_record_offset":44124,"inlinee":6423,"source_line":603,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":44192,"parent_record_offset":44156,"ancestor_record_offset":44124,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 27, 71, 114, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BC8
// 首条位置: {"module":4589,"inlinee":134088}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":54000,"parent_record_offset":53976,"ancestor_record_offset":53976,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":47932,"parent_record_offset":47908,"ancestor_record_offset":47908,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4589,"record_offset":54024,"parent_record_offset":54000,"ancestor_record_offset":53976,"inlinee":125015,"source_line":71,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":19380,"parent_record_offset":19328,"ancestor_record_offset":19304,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":54048,"parent_record_offset":54024,"ancestor_record_offset":53976,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":19352,"parent_record_offset":19328,"ancestor_record_offset":19304,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 27, 71, 114, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BC6
// 首条位置: {"module":4589,"inlinee":134086}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":46044,"parent_record_offset":46012,"ancestor_record_offset":46012,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":55464,"parent_record_offset":55436,"ancestor_record_offset":55436,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":46840,"parent_record_offset":46816,"ancestor_record_offset":46012,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":46864,"parent_record_offset":46840,"ancestor_record_offset":46012,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4589,"record_offset":46816,"parent_record_offset":46640,"ancestor_record_offset":46012,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":46580,"parent_record_offset":46552,"ancestor_record_offset":46012,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":46640,"parent_record_offset":46608,"ancestor_record_offset":46012,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":46608,"parent_record_offset":46580,"ancestor_record_offset":46012,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4589,"record_offset":46676,"parent_record_offset":46640,"ancestor_record_offset":46012,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":46512,"parent_record_offset":46476,"ancestor_record_offset":46012,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":46476,"parent_record_offset":46440,"ancestor_record_offset":46012,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":46404,"parent_record_offset":46368,"ancestor_record_offset":46012,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":46440,"parent_record_offset":46404,"ancestor_record_offset":46012,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4589,"record_offset":46368,"parent_record_offset":46332,"ancestor_record_offset":46012,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4589,"record_offset":46332,"parent_record_offset":46296,"ancestor_record_offset":46012,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4589,"record_offset":46552,"parent_record_offset":46512,"ancestor_record_offset":46012,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":46700,"parent_record_offset":46676,"ancestor_record_offset":46012,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4589,"record_offset":46724,"parent_record_offset":46700,"ancestor_record_offset":46012,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":46940,"parent_record_offset":46916,"ancestor_record_offset":46012,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4589,"record_offset":46772,"parent_record_offset":46748,"ancestor_record_offset":46012,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":46748,"parent_record_offset":46724,"ancestor_record_offset":46012,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4589,"record_offset":46916,"parent_record_offset":46512,"ancestor_record_offset":46012,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":46228,"parent_record_offset":46188,"ancestor_record_offset":46012,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":46188,"parent_record_offset":46152,"ancestor_record_offset":46012,"inlinee":5163,"source_line":445,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":46152,"parent_record_offset":46116,"ancestor_record_offset":46012,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":46116,"parent_record_offset":46080,"ancestor_record_offset":46012,"inlinee":5160,"source_line":139,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":46260,"parent_record_offset":46188,"ancestor_record_offset":46012,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":46296,"parent_record_offset":46260,"ancestor_record_offset":46012,"inlinee":6423,"source_line":603,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":46080,"parent_record_offset":46044,"ancestor_record_offset":46012,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":19040,"parent_record_offset":19016,"ancestor_record_offset":18992,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 27, 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BCA
// 首条位置: {"module":4589,"inlinee":134090}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":18212,"parent_record_offset":18184,"ancestor_record_offset":18184,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":52732,"parent_record_offset":52704,"ancestor_record_offset":52704,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":18240,"parent_record_offset":18212,"ancestor_record_offset":18184,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":18276,"parent_record_offset":18212,"ancestor_record_offset":18184,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$14::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E840
// 首条位置: {"module":4091,"inlinee":124992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":1976,"parent_record_offset":1904,"ancestor_record_offset":1904,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":3676,"parent_record_offset":1904,"ancestor_record_offset":1904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1904,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1904,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1904,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":1904,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":2100,"parent_record_offset":2052,"ancestor_record_offset":1904,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2052,"parent_record_offset":2004,"ancestor_record_offset":1904,"inlinee":124994,"source_line":71,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":1904,"inlinee":71882,"source_line":108,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2868,"parent_record_offset":2832,"ancestor_record_offset":1904,"inlinee":124996,"source_line":114,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3408,"parent_record_offset":3368,"ancestor_record_offset":1904,"inlinee":124998,"source_line":175,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3492,"parent_record_offset":3444,"ancestor_record_offset":1904,"inlinee":125000,"source_line":197,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":1904,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":1904,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4091,"record_offset":2172,"parent_record_offset":2136,"ancestor_record_offset":1904,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":2284,"parent_record_offset":2052,"ancestor_record_offset":1904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$2::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E84A
// 首条位置: {"module":4091,"inlinee":125002}
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
// IPI IDs: 0x1E83D
// 首条位置: {"module":4091,"inlinee":124989}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":932,"parent_record_offset":888,"ancestor_record_offset":888,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":964,"parent_record_offset":932,"ancestor_record_offset":888,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$17::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E842
// 首条位置: {"module":4091,"inlinee":124994}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":2100,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":2284,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":2052,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4091,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":2052,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":2052,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4091,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":2052,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":2052,"inlinee":71882,"source_line":108,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4091,"record_offset":2172,"parent_record_offset":2136,"ancestor_record_offset":2052,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4091,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$4::concrete_default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BD0
// 首条位置: {"module":4590,"inlinee":134096}
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
// 首条位置: {"module":4742,"record_offset":252,"rva":42441760,"va":5411150880}
// 编译实例 RVAs: 0x2879c20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-28303195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411150880,"raw_hex":"e8a52050fe","size":5,"soft_fail":false,"target_va":5382847888,"va":5411151078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4742,"record_offset":380,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04742.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4742,"record_offset":408,"parent_record_offset":380,"ancestor_record_offset":252,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04742.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4742,"record_offset":540,"parent_record_offset":512,"ancestor_record_offset":252,"inlinee":71882,"source_line":108,"archive":"_pdb/B/modules/04742.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4742,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":252,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/04742.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=6
// 源码行: 108, 114, 169, 175, 197, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4592,"record_offset":2096,"rva":40419104,"va":5409128224}
// 编译实例 RVAs: 0x268bf20, 0x268c230, 0x268c580, 0x268c680, 0x268c7a0, 0x268c930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2351628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409129008,"raw_hex":"e80ce22300","size":5,"soft_fail":false,"target_va":5411481328,"va":5409129695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2352820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409128224,"raw_hex":"e8b4e62300","size":5,"soft_fail":false,"target_va":5411481152,"va":5409128327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: config.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34046899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409128224,"raw_hex":"e84d7cf8fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409128398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::__private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-26474871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409128224,"raw_hex":"e889066cfe","size":5,"soft_fail":false,"target_va":5382653424,"va":5409128290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4592,"record_offset":2276,"parent_record_offset":2096,"ancestor_record_offset":2096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4592,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":2096,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4592,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":2096,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4592,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":2096,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4592,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":2096,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4592,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":2096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 108, 114, 175, 197, 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BE0
// 首条位置: {"module":4592,"inlinee":134112}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 108, 169, 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BE3
// 首条位置: {"module":4605,"inlinee":134115}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":5492,"parent_record_offset":5440,"ancestor_record_offset":5440,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":5524,"parent_record_offset":5440,"ancestor_record_offset":5440,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$26::unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":6088,"parent_record_offset":6000,"ancestor_record_offset":6000,"inlinee":71680,"source_line":2135,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":5440,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_unit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":6148,"parent_record_offset":6120,"ancestor_record_offset":6000,"inlinee":71681,"source_line":1905,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":6120,"parent_record_offset":6088,"ancestor_record_offset":6000,"inlinee":6595,"source_line":33,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BDE
// 首条位置: {"module":4592,"inlinee":134110}
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
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4592,"record_offset":2728,"rva":40419360,"va":5409128480}
// 编译实例 RVAs: 0x268c020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2352740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409128480,"raw_hex":"e864e62300","size":5,"soft_fail":false,"target_va":5411481328,"va":5409128583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: config.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34047155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409128480,"raw_hex":"e84d7bf8fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409128654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::__private::string::from_utf8_lossy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-26475127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409128480,"raw_hex":"e889056cfe","size":5,"soft_fail":false,"target_va":5382653424,"va":5409128546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4592,"record_offset":2928,"parent_record_offset":2728,"ancestor_record_offset":2728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4592,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":2728,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4592,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":2728,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4592,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":2728,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4592,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":2728,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4592,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":2728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BE4
// 首条位置: {"module":4605,"inlinee":134116}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4605,"record_offset":31688,"parent_record_offset":31664,"ancestor_record_offset":31664,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4605,"record_offset":31712,"parent_record_offset":31688,"ancestor_record_offset":31664,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4605,"record_offset":31736,"parent_record_offset":31712,"ancestor_record_offset":31664,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4605,"record_offset":31760,"parent_record_offset":31736,"ancestor_record_offset":31664,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BE1
// 首条位置: {"module":4592,"inlinee":134113}
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
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C36
// 首条位置: {"module":4605,"inlinee":134198}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":31352,"parent_record_offset":31324,"ancestor_record_offset":31324,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":31564,"parent_record_offset":31540,"ancestor_record_offset":31324,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":31612,"parent_record_offset":31588,"ancestor_record_offset":31324,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31324,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4605,"record_offset":31492,"parent_record_offset":31468,"ancestor_record_offset":31324,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4605,"record_offset":31468,"parent_record_offset":31352,"ancestor_record_offset":31324,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":31324,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":31516,"parent_record_offset":31492,"ancestor_record_offset":31324,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":32044,"parent_record_offset":31352,"ancestor_record_offset":31324,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":31664,"parent_record_offset":31352,"ancestor_record_offset":31324,"inlinee":134116,"source_line":108,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":31436,"parent_record_offset":31412,"ancestor_record_offset":31324,"inlinee":134113,"source_line":108,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4605,"record_offset":31712,"parent_record_offset":31688,"ancestor_record_offset":31324,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4605,"record_offset":31736,"parent_record_offset":31712,"ancestor_record_offset":31324,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4605,"record_offset":31760,"parent_record_offset":31736,"ancestor_record_offset":31324,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4605,"record_offset":31688,"parent_record_offset":31664,"ancestor_record_offset":31324,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":31808,"parent_record_offset":31352,"ancestor_record_offset":31324,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":31864,"parent_record_offset":31352,"ancestor_record_offset":31324,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":31412,"parent_record_offset":31352,"ancestor_record_offset":31324,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BDC
// 首条位置: {"module":4592,"inlinee":134108}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C34
// 首条位置: {"module":4605,"inlinee":134196}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4605,"record_offset":32268,"parent_record_offset":31092,"ancestor_record_offset":31092,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4605,"record_offset":32528,"parent_record_offset":31092,"ancestor_record_offset":31092,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":32408,"parent_record_offset":31092,"ancestor_record_offset":31092,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":32444,"parent_record_offset":31092,"ancestor_record_offset":31092,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":31148,"parent_record_offset":31092,"ancestor_record_offset":31092,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":32296,"parent_record_offset":31092,"ancestor_record_offset":31092,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4605,"record_offset":32724,"parent_record_offset":32692,"ancestor_record_offset":31092,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4605,"record_offset":32692,"parent_record_offset":32656,"ancestor_record_offset":31092,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":31564,"parent_record_offset":31540,"ancestor_record_offset":31092,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":32756,"parent_record_offset":32656,"ancestor_record_offset":31092,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":32656,"parent_record_offset":32624,"ancestor_record_offset":31092,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":31612,"parent_record_offset":31588,"ancestor_record_offset":31092,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31092,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4605,"record_offset":32624,"parent_record_offset":32592,"ancestor_record_offset":31092,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4605,"record_offset":31492,"parent_record_offset":31468,"ancestor_record_offset":31092,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4605,"record_offset":31468,"parent_record_offset":31352,"ancestor_record_offset":31092,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":31092,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":31516,"parent_record_offset":31492,"ancestor_record_offset":31092,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4605,"record_offset":32044,"parent_record_offset":31352,"ancestor_record_offset":31092,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":31664,"parent_record_offset":31352,"ancestor_record_offset":31092,"inlinee":134116,"source_line":108,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":31436,"parent_record_offset":31412,"ancestor_record_offset":31092,"inlinee":134113,"source_line":108,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":31324,"parent_record_offset":31296,"ancestor_record_offset":31092,"inlinee":134198,"source_line":108,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4605,"record_offset":31712,"parent_record_offset":31688,"ancestor_record_offset":31092,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4605,"record_offset":32560,"parent_record_offset":32528,"ancestor_record_offset":31092,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4605,"record_offset":31260,"parent_record_offset":31220,"ancestor_record_offset":31092,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":32224,"parent_record_offset":31180,"ancestor_record_offset":31092,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4605,"record_offset":31736,"parent_record_offset":31712,"ancestor_record_offset":31092,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4605,"record_offset":31760,"parent_record_offset":31736,"ancestor_record_offset":31092,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4605,"record_offset":31220,"parent_record_offset":31180,"ancestor_record_offset":31092,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4605,"record_offset":31688,"parent_record_offset":31664,"ancestor_record_offset":31092,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":31352,"parent_record_offset":31324,"ancestor_record_offset":31092,"inlinee":71674,"source_line":2039,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$31::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":31180,"parent_record_offset":31148,"ancestor_record_offset":31092,"inlinee":71711,"source_line":2304,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$31::next_value_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":32320,"parent_record_offset":32296,"ancestor_record_offset":31092,"inlinee":71718,"source_line":2317,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":31808,"parent_record_offset":31352,"ancestor_record_offset":31092,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":31864,"parent_record_offset":31352,"ancestor_record_offset":31092,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":31412,"parent_record_offset":31352,"ancestor_record_offset":31092,"inlinee":11530,"source_line":1383,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":31296,"parent_record_offset":31180,"ancestor_record_offset":31092,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":32368,"parent_record_offset":32344,"ancestor_record_offset":31092,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C37
// 首条位置: {"module":4605,"inlinee":134199}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":33400,"parent_record_offset":33176,"ancestor_record_offset":33176,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":33436,"parent_record_offset":33176,"ancestor_record_offset":33176,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":33208,"parent_record_offset":33176,"ancestor_record_offset":33176,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":33352,"parent_record_offset":33264,"ancestor_record_offset":33176,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$29::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4605,"record_offset":33264,"parent_record_offset":33236,"ancestor_record_offset":33176,"inlinee":71723,"source_line":2259,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":33236,"parent_record_offset":33208,"ancestor_record_offset":33176,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":33296,"parent_record_offset":33264,"ancestor_record_offset":33176,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":33320,"parent_record_offset":33296,"ancestor_record_offset":33176,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$20::clone
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118CA
// 首条位置: {"module":1553,"inlinee":71882}
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
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8048
// 首条位置: {"module":575,"inlinee":32840}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":58408,"parent_record_offset":58384,"ancestor_record_offset":58384,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1553,"record_offset":58432,"parent_record_offset":58408,"ancestor_record_offset":58384,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1553,"record_offset":58460,"parent_record_offset":58432,"ancestor_record_offset":58384,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1553,"record_offset":58488,"parent_record_offset":58460,"ancestor_record_offset":58384,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1553,"record_offset":58516,"parent_record_offset":58488,"ancestor_record_offset":58384,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$25::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A23
// 首条位置: {"module":1582,"inlinee":72227}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$25::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1582,"record_offset":44988,"parent_record_offset":44964,"ancestor_record_offset":44964,"inlinee":9032,"source_line":2594,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":45112,"parent_record_offset":45084,"ancestor_record_offset":44964,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1582,"record_offset":45012,"parent_record_offset":44988,"ancestor_record_offset":44964,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1582,"record_offset":45060,"parent_record_offset":45036,"ancestor_record_offset":44964,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1582,"record_offset":45084,"parent_record_offset":45060,"ancestor_record_offset":44964,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1582,"record_offset":45036,"parent_record_offset":45012,"ancestor_record_offset":44964,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 114, 175, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BDF
// 首条位置: {"module":4592,"inlinee":134111}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C2B
// 首条位置: {"module":4605,"inlinee":134187}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":18760,"parent_record_offset":18736,"ancestor_record_offset":18736,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":18812,"parent_record_offset":18760,"ancestor_record_offset":18736,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":18784,"parent_record_offset":18760,"ancestor_record_offset":18736,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BE7
// 首条位置: {"module":4605,"inlinee":134119}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":19584,"parent_record_offset":19560,"ancestor_record_offset":19560,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":19608,"parent_record_offset":19584,"ancestor_record_offset":19560,"inlinee":125015,"source_line":233,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":19636,"parent_record_offset":19584,"ancestor_record_offset":19560,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 114, 175, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C27
// 首条位置: {"module":4605,"inlinee":134183}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4605,"record_offset":16684,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":16648,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":16276,"parent_record_offset":16244,"ancestor_record_offset":16244,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":16504,"parent_record_offset":16476,"ancestor_record_offset":16244,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":16532,"parent_record_offset":16504,"ancestor_record_offset":16244,"inlinee":125015,"source_line":233,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":16476,"parent_record_offset":16448,"ancestor_record_offset":16244,"inlinee":134185,"source_line":197,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":21060,"parent_record_offset":21028,"ancestor_record_offset":20088,"inlinee":134189,"source_line":197,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$2::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":21700,"parent_record_offset":21668,"ancestor_record_offset":20088,"inlinee":134191,"source_line":197,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$3::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":22340,"parent_record_offset":22308,"ancestor_record_offset":20088,"inlinee":134193,"source_line":114,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$4::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":22980,"parent_record_offset":22948,"ancestor_record_offset":20088,"inlinee":134195,"source_line":114,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4605,"record_offset":16388,"parent_record_offset":16364,"ancestor_record_offset":16244,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":4605,"record_offset":16364,"parent_record_offset":16332,"ancestor_record_offset":16244,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":4605,"record_offset":20360,"parent_record_offset":20308,"ancestor_record_offset":20088,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4605,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":16244,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4605,"record_offset":20456,"parent_record_offset":20432,"ancestor_record_offset":20088,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":16608,"parent_record_offset":16332,"ancestor_record_offset":16244,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":16560,"parent_record_offset":16504,"ancestor_record_offset":16244,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4605,"record_offset":20432,"parent_record_offset":20408,"ancestor_record_offset":20088,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4605,"record_offset":20384,"parent_record_offset":20360,"ancestor_record_offset":20088,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4605,"record_offset":20408,"parent_record_offset":20384,"ancestor_record_offset":20088,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":16304,"parent_record_offset":16276,"ancestor_record_offset":16244,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4605,"record_offset":16448,"parent_record_offset":16332,"ancestor_record_offset":16244,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":27192,"parent_record_offset":27164,"ancestor_record_offset":25764,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":21120,"parent_record_offset":21088,"ancestor_record_offset":20088,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$45::next_element_seed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/value.rs
//     evidence={"module":4605,"record_offset":16332,"parent_record_offset":16304,"ancestor_record_offset":16244,"inlinee":7008,"source_line":963,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 114, 175, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C29
// 首条位置: {"module":4605,"inlinee":134185}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":16504,"parent_record_offset":16476,"ancestor_record_offset":16476,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4605,"record_offset":16532,"parent_record_offset":16504,"ancestor_record_offset":16476,"inlinee":125015,"source_line":233,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":16560,"parent_record_offset":16504,"ancestor_record_offset":16476,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 114, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C2D
// 首条位置: {"module":4605,"inlinee":134189}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":21088,"parent_record_offset":21060,"ancestor_record_offset":21060,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":21192,"parent_record_offset":21060,"ancestor_record_offset":21060,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":21148,"parent_record_offset":21088,"ancestor_record_offset":21060,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":21120,"parent_record_offset":21088,"ancestor_record_offset":21060,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$2::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 114, 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C2F
// 首条位置: {"module":4605,"inlinee":134191}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":21728,"parent_record_offset":21700,"ancestor_record_offset":21700,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":21832,"parent_record_offset":21700,"ancestor_record_offset":21700,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":21788,"parent_record_offset":21728,"ancestor_record_offset":21700,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":27192,"parent_record_offset":27164,"ancestor_record_offset":27136,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":21760,"parent_record_offset":21728,"ancestor_record_offset":21700,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$3::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C31
// 首条位置: {"module":4605,"inlinee":134193}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":22368,"parent_record_offset":22340,"ancestor_record_offset":22340,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":22472,"parent_record_offset":22340,"ancestor_record_offset":22340,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":22428,"parent_record_offset":22368,"ancestor_record_offset":22340,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":22400,"parent_record_offset":22368,"ancestor_record_offset":22340,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$4::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20C33
// 首条位置: {"module":4605,"inlinee":134195}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4605,"record_offset":23008,"parent_record_offset":22980,"ancestor_record_offset":22980,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":23112,"parent_record_offset":22980,"ancestor_record_offset":22980,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4605,"record_offset":23068,"parent_record_offset":23008,"ancestor_record_offset":22980,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$99::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4605,"record_offset":23040,"parent_record_offset":23008,"ancestor_record_offset":22980,"inlinee":6681,"source_line":1338,"archive":"_pdb/B/modules/04605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$5::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x212EA
// 首条位置: {"module":4753,"inlinee":135914}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4753,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":1612,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04753.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4753,"record_offset":1664,"parent_record_offset":1636,"ancestor_record_offset":1612,"inlinee":125015,"source_line":233,"archive":"_pdb/B/modules/04753.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4753,"record_offset":1688,"parent_record_offset":1636,"ancestor_record_offset":1612,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04753.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$26::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E844
// 首条位置: {"module":4091,"inlinee":124996}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":2928,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":2868,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":2868,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$6::inherit_from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20B9A
// 首条位置: {"module":4583,"inlinee":134042}
// 编译实例 RVAs: 0x2879de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":5624,"parent_record_offset":5564,"ancestor_record_offset":5564,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4742,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":656,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04742.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4742,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":656,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04742.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4742,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":656,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04742.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$0::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BDA
// 首条位置: {"module":4592,"inlinee":134106}
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
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E846
// 首条位置: {"module":4091,"inlinee":124998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":3444,"parent_record_offset":3408,"ancestor_record_offset":3408,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":3532,"parent_record_offset":3492,"ancestor_record_offset":3408,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$34::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4091,"record_offset":3492,"parent_record_offset":3444,"ancestor_record_offset":3408,"inlinee":125000,"source_line":197,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":3408,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":3408,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$8::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20B9C
// 首条位置: {"module":4583,"inlinee":134044}
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
// IPI IDs: 0x20B9E
// 首条位置: {"module":4583,"inlinee":134046}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$6::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":108129,"source_line":152,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::prefab::config::impl$11::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab/config.rs
//     evidence={"module":4590,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":6760,"inlinee":134048,"source_line":223,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$34::clone
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E848
// 首条位置: {"module":4091,"inlinee":125000}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4091,"record_offset":3532,"parent_record_offset":3492,"ancestor_record_offset":3492,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":3492,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4091,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":3492,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04091.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$10::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BCE
// 首条位置: {"module":4590,"inlinee":134094}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":2496,"parent_record_offset":2452,"ancestor_record_offset":2452,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4590,"record_offset":2564,"parent_record_offset":2532,"ancestor_record_offset":2452,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$20::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":2452,"inlinee":13478,"source_line":419,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$22::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":4590,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2452,"inlinee":13810,"source_line":442,"archive":"_pdb/B/modules/04590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::impl$11::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BA0
// 首条位置: {"module":4583,"inlinee":134048}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4592,"record_offset":7436,"rva":40422000,"va":5409131120}
// 编译实例 RVAs: 0x268ca70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142691640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: config.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409131120,"raw_hex":"e8be4b0000","size":5,"soft_fail":false,"target_va":5409150528,"va":5409131133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142922c60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: config.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2711870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409131120,"raw_hex":"e83e612900","size":5,"soft_fail":false,"target_va":5411843168,"va":5409131293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4592,"record_offset":7708,"parent_record_offset":7436,"ancestor_record_offset":7436,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4592,"record_offset":7884,"parent_record_offset":7436,"ancestor_record_offset":7436,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::content::impl$26::unit_variant
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4592,"record_offset":7744,"parent_record_offset":7436,"ancestor_record_offset":7436,"inlinee":71680,"source_line":2135,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4592,"record_offset":7680,"parent_record_offset":7436,"ancestor_record_offset":7436,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::content::impl$21::deserialize_unit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":4592,"record_offset":7844,"parent_record_offset":7816,"ancestor_record_offset":7436,"inlinee":71681,"source_line":1905,"archive":"_pdb/B/modules/04592.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4592,"record_offset":7816,"parent_record_offset":7788,"ancestor_record_offset":7436,"inlinee":6595,"source_line":33,"archive":"_pdb/B/modules/04592.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "enum2$<asset::prefab::config::MeshProcessorConfig>", "constructor": "enum2$", "arguments": ["asset::prefab::config::MeshProcessorConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> >", "constructor": "core::option::Option", "arguments": ["enum2$<asset::prefab::config::MeshProcessorConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "constructor": "tuple$", "arguments": ["alloc::string::String", "enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "constructor": "std::collections::hash::map::VacantEntry", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > >", "constructor": "ref$", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > > >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::tag::Deserializer>", "enum2$<asset::prefab::config::MeshProcessorConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::PrefabAudioConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<asset::prefab::config::PrefabAudioConfig>", "constructor": "core::option::Option", "arguments": ["asset::prefab::config::PrefabAudioConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> >", "constructor": "enum2$", "arguments": ["core::option::Option<asset::prefab::config::PrefabAudioConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >", "constructor": "asset::inheritable::impl$1::inherit_from", "arguments": ["enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabAudioConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::prefab::config::PrefabConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "ref$<asset::prefab::config::PrefabConfig>", "constructor": "ref$", "arguments": ["asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> >", "constructor": "tuple$", "arguments": ["ref$<asset::prefab::PrefabName>", "ref$<asset::prefab::config::PrefabConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > >", "constructor": "ref$", "arguments": ["tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >", "constructor": "tuple$", "arguments": ["ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >", "country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "constructor": "tuple$", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["asset::prefab::PrefabName", "asset::prefab::config::PrefabConfig", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<alloc::string::String,asset::prefab::config::PrefabPartConfig>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>", "constructor": "asset::inheritable::impl$5::inherit_from", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["alloc::string::String", "asset::prefab::config::PrefabPartConfig", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<asset::prefab::config::PrefabRtConfig>", "constructor": "core::option::Option", "arguments": ["asset::prefab::config::PrefabRtConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> >", "constructor": "enum2$", "arguments": ["core::option::Option<asset::prefab::config::PrefabRtConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >", "constructor": "asset::inheritable::impl$1::inherit_from", "arguments": ["enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabRtConfig>", "constructor": "asset::inheritable::deserialize_inheritable_option", "arguments": ["ref_mut$<ron::de::tag::Deserializer>", "asset::prefab::config::PrefabRtConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 候选参数类型: ["ref$<enum2$<serde::__private::de::content::Content> >"]

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize
// 信息 ID: 9b01117b6ee75671c76e4a74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e55b91db0419e2d4eecc727d: serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<asset::prefab::config::PrefabConfig>,asset::prefab::config::PrefabConfig>; module=4094, inline_record=1388; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::prefab::config::PrefabPartConfig,std::hash::random::RandomState> >; module=4589, inline_record=17204; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=54024; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >; module=4590, inline_record=8096; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >; module=4590, inline_record=8152; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,enum2$<asset::prefab::config::MeshProcessorConfig> >; module=4590, inline_record=8332; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>; module=4590, inline_record=9876; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::deserialize_inheritable_option<ref_mut$<ron::de::tag::Deserializer>,asset::prefab::config::PrefabAudioConfig>; module=4590, inline_record=9932; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::expecting
// 信息 ID: 63853c989af1376b4e1c7936
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4592, inline_record=424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 信息 ID: 6eabb66357ba12c9953b1bf6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_bytes -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 96f16edb3402f61e144a189d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=40060; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=43320; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_identifier<asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4589, inline_record=59012; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=4605, inline_record=1704; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=4605, inline_record=2056; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4605, inline_record=29256; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4605, inline_record=29612; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4605, inline_record=30236; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 9b86e4b92a3c3688829ada07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2687a83 -> 0x268be70 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<asset::prefab::config::_::impl$0::deserialize::__FieldVisitor> @ 0x287db9a -> 0x268be70 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<asset::prefab::config::_::impl$0::deserialize::__FieldVisitor> @ 0x287dbd0 -> 0x268be70 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor> @ 0x2694848 -> 0x268c120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$0::visit_u64
// 信息 ID: dbf2f5e6e92e6dc87f3baa6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=4605, inline_record=1480; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$25::variant_seed<enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::__Field> >; module=4605, inline_record=1840; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4605, inline_record=28248; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4605, inline_record=28472; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4605, inline_record=29032; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4605, inline_record=29392; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_identifier<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__FieldVisitor>; module=4605, inline_record=30372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: b92a227b47c2a3af43bedba7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=39640; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=39796; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=39920; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=42900; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=43056; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=43160; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=51296; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=51452; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 1562b3c2d9eada803d71047f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4592, inline_record=1048; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4592, inline_record=1240; 不是独立调用边。
// [PDB 内联宿主] serde::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4592, inline_record=1432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 13ed3d3c14cd65ac0a6ebc28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=5440; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=6000; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=7168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::expecting
// 信息 ID: caf1ad6c9163a59601947083
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=4592, inline_record=856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes<enum2$<ron::error::Error> >
// 信息 ID: 97a0039142f495b07443100d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_bytes -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: acdfab6e7e5824d1bc3c6be9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=4605, inline_record=31664; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=4605, inline_record=31892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_u64
// 信息 ID: 3ca282d56e1943629abf1ef6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=4605, inline_record=31436; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=4605, inline_record=31836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 5eec83718fc4735cc2e68056
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=4605, inline_record=31324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::expecting
// 信息 ID: e78feca5e0b791d0e92c4d16
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=4592, inline_record=636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 8812a040e45d8df51e908222
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=4605, inline_record=31092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_seq
// 信息 ID: 593aa21ab82614b92e88fb17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$26::struct_variant<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=4605, inline_record=33176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> > >
// 信息 ID: f735e4dcce5dbe6d719ecfe1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] serde::__private::de::content::EnumRefDeserializer -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// [类型关系] asset::prefab::config::_::impl$0::deserialize::impl$2::visit_enum -> ["serde::__private::de::content::EnumRefDeserializer<enum2$<ron::error::Error> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::__private::de::content::impl$21::deserialize_enum<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor> @ 0x2691f43 -> 0x268ca70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: cdac3c59af211505eded075b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=39260; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=42532; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=50920; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=14204; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=17620; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=23756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: 3b2f0adee09fb99387bd992d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=44124; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=52088; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=14732; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=18468; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=24612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 信息 ID: 6774f49238a0d08b76a86e15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=47908; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=53976; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=19304; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=24288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
// 信息 ID: 0b1dd108ca4b9c7abeabbde3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=46012; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=55436; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=18992; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=24880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
// 信息 ID: b4c978110c235fd1d8c079bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=52704; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=18184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$4::deserialize
// 信息 ID: f8da5a9d2d872f8f27eff10b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=18736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_map::impl$5::deserialize
// 信息 ID: c0a548317ef1de670833a24c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=19560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 70c09ae7d72fa60d9cb34fb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=16244; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=20088; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=25764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$0::deserialize
// 信息 ID: 3ec477f228b00cd607e0b657
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=16476; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=20536; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=26132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$1::deserialize
// 信息 ID: de0a562fc5f2fd13e12df8fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=21060; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=26628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$2::deserialize
// 信息 ID: ed64c42ee5ac8f21088e75ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=21700; 不是独立调用边。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=27136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$3::deserialize
// 信息 ID: 102b8b38e03d1b5a8cf43fd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=22340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$4::deserialize
// 信息 ID: 848f9608a80dd7a161cf114d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::__private::de::content::impl$21::deserialize_struct<enum2$<ron::error::Error>,asset::prefab::config::_::impl$0::deserialize::__Visitor>; module=4605, inline_record=22980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::impl$5::deserialize
// 信息 ID: 1885b78d7d0bbeb1e0916a3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e55b91db0419e2d4eecc727d: serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::value::impl$45::next_element_seed<core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<serde::__private::de::content::Content> >,serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> > (*)(ref$<enum2$<serde::__private::de::content::Content> >)>,serde::__private::de::content::ContentRefDeserializer<enum2$<ron::error::Error> >,enum2$<ron::error::Error>,core::marker::PhantomData<asset::prefab::config::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith> >; module=4753, inline_record=1612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$10::inherit_from
// 信息 ID: d6032ffb353e6cc086cd15cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>; module=4590, inline_record=2452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$11::concrete_default
// 信息 ID: bfdf2765f0f9b7594dc0e2a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4590, inline_record=6808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$13::default
// 信息 ID: 0a75a31ce3646113be755a90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree; module=3085, inline_record=4704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$14::clone
// 信息 ID: 63b7ad325e25bdf001936f80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=1904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$17::clone
// 信息 ID: 67f45ec3172834dddafbed8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=2052; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>; module=4590, inline_record=3580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$1::inherit_from
// 信息 ID: 7873065ce015ce5564f62dfa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=3085, inline_record=9900; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::scan::scan_asset_tree_walker; module=3085, inline_record=13260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$20::clone
// 信息 ID: 468a668c11e602ec9972df08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=57084; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=57148; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=2204; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig>; module=4590, inline_record=3744; 不是独立调用边。
// [PDB 内联宿主] asset::prefab::config::impl$5::inherit_from; module=4742, inline_record=540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$23::eq
// 信息 ID: 776334cc4e05f22a82b69ba5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::load_prefabs_impl; module=1553, inline_record=58384; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>; module=1613, inline_record=39624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$25::hash
// 信息 ID: 72f69ecc2cf44dcfdee2d996
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > > >; module=1582, inline_record=44964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$26::clone
// 信息 ID: 7e24337e879826b39dc7a9a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=2868; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >; module=4590, inline_record=5912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$2::concrete_default
// 信息 ID: b668c78260f853c718acee4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=6228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$31::clone
// 信息 ID: b767d4886e3cf296e74cc5e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=3408; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4590, inline_record=6944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$34::clone
// 信息 ID: a03b38502b749abe65082c6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=3492; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<asset::prefab::config::AudioTriggerConfig>; module=4590, inline_record=2780; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4590, inline_record=7016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$3::inherit_from
// 信息 ID: baab4469229a07b620db4ec1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=888; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$5::inherit_from<alloc::string::String,asset::prefab::config::PrefabPartConfig>; module=4091, inline_record=6256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$4::concrete_default
// 信息 ID: 79b922be789a699bf341bacc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$5::inherit_from
// 信息 ID: fd9a66992e61d9f1d4d740a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<asset::prefab::config::PrefabRenderConfig> @ 0x2689f54 -> 0x2879c20 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$6::inherit_from
// 信息 ID: 663ef840c5f40bc4cb4a2af0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > > @ 0x268a346 -> 0x2879de0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabRtConfig> > >; module=4590, inline_record=5564; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$8::inherit_from
// 信息 ID: 527b47713e490d318844ecdf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4590, inline_record=6732; 不是独立调用边。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4590, inline_record=6844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::prefab::config::impl$9::concrete_default
// 信息 ID: 1f27df2e2735f68d6934875a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::inheritable::impl$1::inherit_from<enum2$<core::option::Option<asset::prefab::config::PrefabAudioConfig> > >; module=4590, inline_record=6760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
