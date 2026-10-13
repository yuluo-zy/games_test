// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/clearing.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f2977904930e14180d154fcb699ae69b12e39f6f4f9838562535e9b807934f75
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: clearing.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 538, 554, 1585, 1600
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use libfmod; // namespace candidate
//   package identities: [{"name":"libfmod","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.222.6"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.9.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::util_add_clearing_shader
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B94
// 首条位置: {"module":554,"inlinee":31636}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":554,"record_offset":16900,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":17068,"parent_record_offset":16856,"ancestor_record_offset":16856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":17244,"parent_record_offset":17220,"ancestor_record_offset":16856,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":17220,"parent_record_offset":17164,"ancestor_record_offset":16856,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":554,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":16856,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":17192,"parent_record_offset":17164,"ancestor_record_offset":16856,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":17164,"parent_record_offset":17140,"ancestor_record_offset":16856,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":17140,"parent_record_offset":17116,"ancestor_record_offset":16856,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":16924,"parent_record_offset":16900,"ancestor_record_offset":16856,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":17092,"parent_record_offset":17068,"ancestor_record_offset":16856,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78D8
// 首条位置: {"module":1600,"inlinee":30936}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":203108,"rva":10569248,"va":5379278368}
// 编译实例 RVAs: 0xa14620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":538,"record_offset":203292,"parent_record_offset":203108,"ancestor_record_offset":203108,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":538,"record_offset":203312,"parent_record_offset":203292,"ancestor_record_offset":203108,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":538,"record_offset":203332,"parent_record_offset":203312,"ancestor_record_offset":203108,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":538,"record_offset":203352,"parent_record_offset":203332,"ancestor_record_offset":203108,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BB3
// 首条位置: {"module":1600,"inlinee":72627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":224576,"parent_record_offset":224552,"ancestor_record_offset":224552,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":224724,"parent_record_offset":224600,"ancestor_record_offset":224552,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":224640,"parent_record_offset":224600,"ancestor_record_offset":224552,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":224600,"parent_record_offset":224576,"ancestor_record_offset":224552,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":224664,"parent_record_offset":224640,"ancestor_record_offset":224552,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":224688,"parent_record_offset":224664,"ancestor_record_offset":224552,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78C5
// 首条位置: {"module":538,"inlinee":30917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BB1
// 首条位置: {"module":1600,"inlinee":72625}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1600,"record_offset":224808,"parent_record_offset":223748,"ancestor_record_offset":223748,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1600,"record_offset":226920,"parent_record_offset":223748,"ancestor_record_offset":223748,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":226888,"parent_record_offset":223748,"ancestor_record_offset":223748,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1600,"record_offset":227328,"parent_record_offset":223748,"ancestor_record_offset":223748,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":223824,"parent_record_offset":223748,"ancestor_record_offset":223748,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":224836,"parent_record_offset":223748,"ancestor_record_offset":223748,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1600,"record_offset":227264,"parent_record_offset":227240,"ancestor_record_offset":223748,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1600,"record_offset":227240,"parent_record_offset":227216,"ancestor_record_offset":223748,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":227676,"parent_record_offset":227648,"ancestor_record_offset":223748,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":227216,"parent_record_offset":227192,"ancestor_record_offset":223748,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":227192,"parent_record_offset":227168,"ancestor_record_offset":223748,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1600,"record_offset":224724,"parent_record_offset":224600,"ancestor_record_offset":223748,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1600,"record_offset":227072,"parent_record_offset":227044,"ancestor_record_offset":223748,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":224768,"parent_record_offset":223872,"ancestor_record_offset":223748,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/clearing.rs
//     evidence={"module":1600,"record_offset":224552,"parent_record_offset":224528,"ancestor_record_offset":223748,"inlinee":72627,"source_line":56,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$56::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":1600,"record_offset":226848,"parent_record_offset":226824,"ancestor_record_offset":223748,"inlinee":31418,"source_line":2081,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1600,"record_offset":227400,"parent_record_offset":227376,"ancestor_record_offset":223748,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":227488,"parent_record_offset":227464,"ancestor_record_offset":223748,"inlinee":6721,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":227376,"parent_record_offset":227352,"ancestor_record_offset":223748,"inlinee":5157,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":224528,"parent_record_offset":223872,"ancestor_record_offset":223748,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1600,"record_offset":225308,"parent_record_offset":225284,"ancestor_record_offset":223748,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":224128,"parent_record_offset":223956,"ancestor_record_offset":223748,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":224920,"parent_record_offset":224876,"ancestor_record_offset":223748,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":223956,"parent_record_offset":223872,"ancestor_record_offset":223748,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":224016,"parent_record_offset":223956,"ancestor_record_offset":223748,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":223872,"parent_record_offset":223824,"ancestor_record_offset":223748,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":224876,"parent_record_offset":224836,"ancestor_record_offset":223748,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":224600,"parent_record_offset":224576,"ancestor_record_offset":223748,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1600,"record_offset":224576,"parent_record_offset":224552,"ancestor_record_offset":223748,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":224152,"parent_record_offset":224128,"ancestor_record_offset":223748,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":224048,"parent_record_offset":224016,"ancestor_record_offset":223748,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":224176,"parent_record_offset":224152,"ancestor_record_offset":223748,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":224080,"parent_record_offset":224048,"ancestor_record_offset":223748,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11BB4
// 首条位置: {"module":1600,"inlinee":72628}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1600,"record_offset":230120,"parent_record_offset":229220,"ancestor_record_offset":229220,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1600,"record_offset":229312,"parent_record_offset":229220,"ancestor_record_offset":229220,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":229284,"parent_record_offset":229220,"ancestor_record_offset":229220,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1600,"record_offset":230380,"parent_record_offset":230356,"ancestor_record_offset":229220,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1600,"record_offset":230356,"parent_record_offset":230280,"ancestor_record_offset":229220,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":230320,"parent_record_offset":230280,"ancestor_record_offset":229220,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":230280,"parent_record_offset":230248,"ancestor_record_offset":229220,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1600,"record_offset":230248,"parent_record_offset":230216,"ancestor_record_offset":229220,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1600,"record_offset":230152,"parent_record_offset":230120,"ancestor_record_offset":229220,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1600,"record_offset":230084,"parent_record_offset":229412,"ancestor_record_offset":229220,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":229656,"parent_record_offset":229500,"ancestor_record_offset":229220,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":229500,"parent_record_offset":229412,"ancestor_record_offset":229220,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":229552,"parent_record_offset":229500,"ancestor_record_offset":229220,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":1600,"record_offset":229412,"parent_record_offset":229368,"ancestor_record_offset":229220,"inlinee":6533,"source_line":1916,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":229680,"parent_record_offset":229656,"ancestor_record_offset":229220,"inlinee":6488,"source_line":642,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":229580,"parent_record_offset":229552,"ancestor_record_offset":229220,"inlinee":6487,"source_line":637,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":229704,"parent_record_offset":229680,"ancestor_record_offset":229220,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":1600,"record_offset":229608,"parent_record_offset":229580,"ancestor_record_offset":229220,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/01600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":8628,"rva":10980000,"va":5379689120}
// 编译实例 RVAs: 0xa78aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2955132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e87c172d00","size":5,"soft_fail":false,"target_va":5382644528,"va":5379689391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4793364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8ecdbb6ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5379694095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4791332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8dce3b6ff","size":5,"soft_fail":false,"target_va":5374904480,"va":5379695807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4608842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8b6acb9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379690341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4173190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e87a52c0ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5379689217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2919707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8e572d3ff","size":5,"soft_fail":false,"target_va":5376776208,"va":5379695910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140879230
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2098259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8adfbdfff","size":5,"soft_fail":false,"target_va":5377593904,"va":5379692158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2033829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e85bf7e0ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5379691472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1970842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e866ede1ff","size":5,"soft_fail":false,"target_va":5377723536,"va":5379694373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409365d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1322130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e86ed3ebff","size":5,"soft_fail":false,"target_va":5378368976,"va":5379691101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a123a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-422089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8378ff9ff","size":5,"soft_fail":false,"target_va":5379269536,"va":5379691620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0eeb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t615093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8b5620900","size":5,"soft_fail":false,"target_va":5380304560,"va":5379689462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0fe60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t617268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8346b0900","size":5,"soft_fail":false,"target_va":5380308576,"va":5379691303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3fe40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t815437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e84d710c00","size":5,"soft_fail":false,"target_va":5380505152,"va":5379689710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b40210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t813446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e886690c00","size":5,"soft_fail":false,"target_va":5380506128,"va":5379692677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1529976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e878581700","size":5,"soft_fail":false,"target_va":5381222304,"va":5379692323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3137476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8c4df2f00","size":5,"soft_fail":false,"target_va":5382827728,"va":5379690247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412612d0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8290051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8037f7e00","size":5,"soft_fail":false,"target_va":5387981520,"va":5379691464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c7380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8707287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8d7dc8400","size":5,"soft_fail":false,"target_va":5388399488,"va":5379692196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e68e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8836766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e89ed68600","size":5,"soft_fail":false,"target_va":5388527840,"va":5379691069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e6ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8837043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8b3d78600","size":5,"soft_fail":false,"target_va":5388528336,"va":5379691288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fa010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8916264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8280d8800","size":5,"soft_fail":false,"target_va":5388607504,"va":5379691235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414062b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10015505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e811d39800","size":5,"soft_fail":false,"target_va":5389705904,"va":5379690394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141556080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11390472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e808cead00","size":5,"soft_fail":false,"target_va":5391081600,"va":5379691123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3147674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e89a073000","size":5,"soft_fail":false,"target_va":5382840208,"va":5379692529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31655331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8a305e301","size":5,"soft_fail":false,"target_va":5411347040,"va":5379691704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4367747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e87d5abdff","size":5,"soft_fail":false,"target_va":5375322928,"va":5379690670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2960213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8552b2d00","size":5,"soft_fail":false,"target_va":5382655024,"va":5379694806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8809781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8356d8600","size":5,"soft_fail":false,"target_va":5388499888,"va":5379690102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2734491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e89bb92900","size":5,"soft_fail":false,"target_va":5382424224,"va":5379689728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t31849523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e833fce501","size":5,"soft_fail":false,"target_va":5411542880,"va":5379693352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-464663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8e9e8f8ff","size":5,"soft_fail":false,"target_va":5379226784,"va":5379691442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::glade_themes::flowery::startup_flowery
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11379497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e829a3ad00","size":5,"soft_fail":false,"target_va":5391069472,"va":5379689970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::glade_themes::olden::startup_olden
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11381607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e867abad00","size":5,"soft_fail":false,"target_va":5391071696,"va":5379690084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t663539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8f31f0a00","size":5,"soft_fail":false,"target_va":5380353184,"va":5379689640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3015156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e8f4012e00","size":5,"soft_fail":false,"target_va":5382705472,"va":5379690311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3053836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379689120,"raw_hex":"e80c992e00","size":5,"soft_fail":false,"target_va":5382744080,"va":5379690239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$90::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":554,"record_offset":12968,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":26646,"source_line":2533,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":11896,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":12752,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":13424,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":554,"record_offset":12808,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":554,"record_offset":9940,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":554,"record_offset":10000,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4958,"source_line":872,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":554,"record_offset":9352,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":554,"record_offset":16768,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":554,"record_offset":8956,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":13552,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":11356,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":554,"record_offset":11268,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":554,"record_offset":9276,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":554,"record_offset":13340,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":13700,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":554,"record_offset":9164,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":554,"record_offset":16560,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9812,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":26641,"source_line":269,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9692,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":31632,"source_line":289,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_olden
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9748,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":24820,"source_line":284,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9220,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":7619,"source_line":274,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::InteractableForestPlacements::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":554,"record_offset":11984,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":31238,"source_line":110,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::clearing::util_add_clearing_shader
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/clearing.rs
//     evidence={"module":554,"record_offset":16856,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":31636,"source_line":41,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":554,"record_offset":8988,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":554,"record_offset":8872,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":554,"record_offset":10440,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":554,"record_offset":10408,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":554,"record_offset":10268,"parent_record_offset":8628,"ancestor_record_offset":8628,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":10888,"parent_record_offset":10864,"ancestor_record_offset":8628,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":10864,"parent_record_offset":10808,"ancestor_record_offset":8628,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":554,"record_offset":16900,"parent_record_offset":16856,"ancestor_record_offset":8628,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":554,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":8628,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":12104,"parent_record_offset":12080,"ancestor_record_offset":8628,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":9536,"parent_record_offset":9508,"ancestor_record_offset":8628,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":9620,"parent_record_offset":9596,"ancestor_record_offset":8628,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":10836,"parent_record_offset":10808,"ancestor_record_offset":8628,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":8628,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":9596,"parent_record_offset":9564,"ancestor_record_offset":8628,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":8628,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":12128,"parent_record_offset":12104,"ancestor_record_offset":8628,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":9564,"parent_record_offset":9536,"ancestor_record_offset":8628,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":10784,"parent_record_offset":10760,"ancestor_record_offset":8628,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":12348,"parent_record_offset":12308,"ancestor_record_offset":8628,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":9188,"parent_record_offset":9164,"ancestor_record_offset":8628,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":12308,"parent_record_offset":12280,"ancestor_record_offset":8628,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":8628,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":12056,"parent_record_offset":12020,"ancestor_record_offset":8628,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":12020,"parent_record_offset":11984,"ancestor_record_offset":8628,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":8628,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":14216,"parent_record_offset":13924,"ancestor_record_offset":8628,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":9508,"parent_record_offset":9472,"ancestor_record_offset":8628,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":14456,"parent_record_offset":13924,"ancestor_record_offset":8628,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":13580,"parent_record_offset":13552,"ancestor_record_offset":8628,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":554,"record_offset":13900,"parent_record_offset":13848,"ancestor_record_offset":8628,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":554,"record_offset":9968,"parent_record_offset":9940,"ancestor_record_offset":8628,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":554,"record_offset":9440,"parent_record_offset":9412,"ancestor_record_offset":8628,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":554,"record_offset":9472,"parent_record_offset":9440,"ancestor_record_offset":8628,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":554,"record_offset":9384,"parent_record_offset":9352,"ancestor_record_offset":8628,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":554,"record_offset":9412,"parent_record_offset":9384,"ancestor_record_offset":8628,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":554,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":8628,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":554,"record_offset":13968,"parent_record_offset":13924,"ancestor_record_offset":8628,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":554,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":8628,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":554,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":8628,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":14624,"parent_record_offset":14596,"ancestor_record_offset":8628,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":16924,"parent_record_offset":16900,"ancestor_record_offset":8628,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":554,"record_offset":9044,"parent_record_offset":9012,"ancestor_record_offset":8628,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":554,"record_offset":9072,"parent_record_offset":9012,"ancestor_record_offset":8628,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=78
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":9320,"parent_record_offset":9276,"ancestor_record_offset":8628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":12776,"parent_record_offset":12752,"ancestor_record_offset":8628,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":12880,"parent_record_offset":12808,"ancestor_record_offset":8628,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":12380,"parent_record_offset":12348,"ancestor_record_offset":8628,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":12908,"parent_record_offset":12808,"ancestor_record_offset":8628,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":12848,"parent_record_offset":12808,"ancestor_record_offset":8628,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":8628,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":554,"record_offset":16724,"parent_record_offset":16592,"ancestor_record_offset":8628,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":554,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":8628,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":554,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":8628,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":554,"record_offset":16640,"parent_record_offset":16592,"ancestor_record_offset":8628,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":554,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":8628,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":13756,"parent_record_offset":13732,"ancestor_record_offset":8628,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":13996,"parent_record_offset":13968,"ancestor_record_offset":8628,"inlinee":11874,"source_line":313,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":14036,"parent_record_offset":13996,"ancestor_record_offset":8628,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":13732,"parent_record_offset":13700,"ancestor_record_offset":8628,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":554,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":8628,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":8628,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":8628,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":12408,"parent_record_offset":12348,"ancestor_record_offset":8628,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":12440,"parent_record_offset":12308,"ancestor_record_offset":8628,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":554,"record_offset":16592,"parent_record_offset":16560,"ancestor_record_offset":8628,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9868,"parent_record_offset":9840,"ancestor_record_offset":8628,"inlinee":26645,"source_line":53,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_flowery
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":8628,"inlinee":31633,"source_line":102,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_olden
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9776,"parent_record_offset":9748,"ancestor_record_offset":8628,"inlinee":31634,"source_line":94,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":8628,"inlinee":26885,"source_line":118,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":554,"record_offset":9840,"parent_record_offset":9812,"ancestor_record_offset":8628,"inlinee":26643,"source_line":198,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":554,"record_offset":9100,"parent_record_offset":9012,"ancestor_record_offset":8628,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":554,"record_offset":9012,"parent_record_offset":8988,"ancestor_record_offset":8628,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":554,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":8628,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":554,"record_offset":16668,"parent_record_offset":16592,"ancestor_record_offset":8628,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":554,"record_offset":10316,"parent_record_offset":10292,"ancestor_record_offset":8628,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":554,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":8628,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":554,"record_offset":10340,"parent_record_offset":10316,"ancestor_record_offset":8628,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":554,"record_offset":9904,"parent_record_offset":9840,"ancestor_record_offset":8628,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":554,"record_offset":10292,"parent_record_offset":10268,"ancestor_record_offset":8628,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":554,"record_offset":10496,"parent_record_offset":10472,"ancestor_record_offset":8628,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":554,"record_offset":10472,"parent_record_offset":10440,"ancestor_record_offset":8628,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78C8
// 首条位置: {"module":538,"inlinee":30920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":1585,"record_offset":154124,"parent_record_offset":154052,"ancestor_record_offset":154052,"inlinee":26267,"source_line":10,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1585,"record_offset":154096,"parent_record_offset":154052,"ancestor_record_offset":154052,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1585,"record_offset":154152,"parent_record_offset":154124,"ancestor_record_offset":154052,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":154188,"parent_record_offset":154152,"ancestor_record_offset":154052,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":154212,"parent_record_offset":154188,"ancestor_record_offset":154052,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::init_clearing::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78C7
// 首条位置: {"module":538,"inlinee":30919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::placement_data::PlacementDataSsbo::to_transform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/placement_data.rs
//     evidence={"module":538,"record_offset":185388,"parent_record_offset":185260,"ancestor_record_offset":185260,"inlinee":26275,"source_line":75,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":538,"record_offset":186056,"parent_record_offset":185260,"ancestor_record_offset":185260,"inlinee":25215,"source_line":372,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":185296,"parent_record_offset":185260,"ancestor_record_offset":185260,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":538,"record_offset":185596,"parent_record_offset":185260,"ancestor_record_offset":185260,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":538,"record_offset":186192,"parent_record_offset":186096,"ancestor_record_offset":185260,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":538,"record_offset":185848,"parent_record_offset":185796,"ancestor_record_offset":185260,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":538,"record_offset":185820,"parent_record_offset":185796,"ancestor_record_offset":185260,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":538,"record_offset":185556,"parent_record_offset":185532,"ancestor_record_offset":185260,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":538,"record_offset":185620,"parent_record_offset":185596,"ancestor_record_offset":185260,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":538,"record_offset":185444,"parent_record_offset":185416,"ancestor_record_offset":185260,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":538,"record_offset":185652,"parent_record_offset":185620,"ancestor_record_offset":185260,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":538,"record_offset":185416,"parent_record_offset":185388,"ancestor_record_offset":185260,"inlinee":24914,"source_line":136,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":538,"record_offset":185704,"parent_record_offset":185652,"ancestor_record_offset":185260,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":538,"record_offset":185796,"parent_record_offset":185620,"ancestor_record_offset":185260,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":538,"record_offset":186408,"parent_record_offset":186056,"ancestor_record_offset":185260,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":185320,"parent_record_offset":185296,"ancestor_record_offset":185260,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":538,"record_offset":185532,"parent_record_offset":185416,"ancestor_record_offset":185260,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":538,"record_offset":185492,"parent_record_offset":185468,"ancestor_record_offset":185260,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":538,"record_offset":185468,"parent_record_offset":185444,"ancestor_record_offset":185260,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":538,"record_offset":185352,"parent_record_offset":185320,"ancestor_record_offset":185260,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::clearing::uninit_clearing
// 记录形式: compiled_procedure; instances=1
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":20752,"rva":10986928,"va":5379696048}
// 编译实例 RVAs: 0xa7a5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a680
//     owner=unresolved_namespace; count=1
//     target_aliases=24 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9635052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e814fb6cff","size":5,"soft_fail":false,"target_va":5370062464,"va":5379697511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4615240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e8b893b9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379696723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4615529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e89792b9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379697028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089a280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1967853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e813f9e1ff","size":5,"soft_fail":false,"target_va":5377729152,"va":5379697000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-164857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e8077cfdff","size":5,"soft_fail":false,"target_va":5379532464,"va":5379697316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b794c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1044059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e85bee0f00","size":5,"soft_fail":false,"target_va":5380740288,"va":5379696224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b79ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1045695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e8bff40f00","size":5,"soft_fail":false,"target_va":5380741840,"va":5379696140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b79d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1046315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e82bf70f00","size":5,"soft_fail":false,"target_va":5380742448,"va":5379696128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141559330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11398373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e8e5ecad00","size":5,"soft_fail":false,"target_va":5391094576,"va":5379696198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: clearing.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32150810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e81a95ea01","size":5,"soft_fail":false,"target_va":5411847200,"va":5379696385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31843781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e8c5e5e501","size":5,"soft_fail":false,"target_va":5411540963,"va":5379697177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4373570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e8be43bdff","size":5,"soft_fail":false,"target_va":5375322928,"va":5379696493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2957467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e89b202d00","size":5,"soft_fail":false,"target_va":5382655024,"va":5379697552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31846179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e823efe501","size":5,"soft_fail":false,"target_va":5411542880,"va":5379696696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-156610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e83e9cfdff","size":5,"soft_fail":false,"target_va":5379540032,"va":5379696637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10209758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379696048,"raw_hex":"e8dec99b00","size":5,"soft_fail":false,"target_va":5389906640,"va":5379696877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":23472,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":554,"record_offset":20932,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":23632,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":24536,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":554,"record_offset":24292,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":23556,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":554,"record_offset":21184,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":31637,"source_line":76,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/placement_data_library.rs
//     evidence={"module":554,"record_offset":20988,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":26800,"source_line":41,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorld::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":554,"record_offset":21700,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":29956,"source_line":539,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":554,"record_offset":23688,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":27239,"source_line":321,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":554,"record_offset":24368,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":27234,"source_line":505,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::RtWorld::reset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":554,"record_offset":21552,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":27400,"source_line":147,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::InteractableForestPlacements::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":554,"record_offset":22660,"parent_record_offset":20752,"ancestor_record_offset":20752,"inlinee":31238,"source_line":110,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":24104,"parent_record_offset":24080,"ancestor_record_offset":20752,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":24128,"parent_record_offset":24104,"ancestor_record_offset":20752,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":24768,"parent_record_offset":24744,"ancestor_record_offset":20752,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":24080,"parent_record_offset":24052,"ancestor_record_offset":20752,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":24744,"parent_record_offset":24688,"ancestor_record_offset":20752,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":22780,"parent_record_offset":22756,"ancestor_record_offset":20752,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":20752,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":24716,"parent_record_offset":24688,"ancestor_record_offset":20752,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":24688,"parent_record_offset":24664,"ancestor_record_offset":20752,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":22828,"parent_record_offset":22804,"ancestor_record_offset":20752,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":22804,"parent_record_offset":22780,"ancestor_record_offset":20752,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":24052,"parent_record_offset":24020,"ancestor_record_offset":20752,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":24020,"parent_record_offset":23992,"ancestor_record_offset":20752,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":24664,"parent_record_offset":24640,"ancestor_record_offset":20752,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":23928,"parent_record_offset":23900,"ancestor_record_offset":20752,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":23900,"parent_record_offset":23872,"ancestor_record_offset":20752,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":23844,"parent_record_offset":23816,"ancestor_record_offset":20752,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":23872,"parent_record_offset":23844,"ancestor_record_offset":20752,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":23816,"parent_record_offset":23788,"ancestor_record_offset":20752,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":554,"record_offset":23788,"parent_record_offset":23688,"ancestor_record_offset":20752,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":23024,"parent_record_offset":22984,"ancestor_record_offset":20752,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":23760,"parent_record_offset":23688,"ancestor_record_offset":20752,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":22984,"parent_record_offset":22956,"ancestor_record_offset":20752,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":22756,"parent_record_offset":22732,"ancestor_record_offset":20752,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":22732,"parent_record_offset":22696,"ancestor_record_offset":20752,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":22276,"parent_record_offset":21768,"ancestor_record_offset":20752,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":23964,"parent_record_offset":23928,"ancestor_record_offset":20752,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":23496,"parent_record_offset":23472,"ancestor_record_offset":20752,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":554,"record_offset":24260,"parent_record_offset":23688,"ancestor_record_offset":20752,"inlinee":27241,"source_line":647,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":554,"record_offset":20956,"parent_record_offset":20932,"ancestor_record_offset":20752,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":554,"record_offset":24204,"parent_record_offset":24180,"ancestor_record_offset":20752,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":554,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":20752,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":554,"record_offset":22312,"parent_record_offset":21700,"ancestor_record_offset":20752,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$59::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":554,"record_offset":22336,"parent_record_offset":22312,"ancestor_record_offset":20752,"inlinee":13694,"source_line":906,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":554,"record_offset":22388,"parent_record_offset":22364,"ancestor_record_offset":20752,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":554,"record_offset":22364,"parent_record_offset":22336,"ancestor_record_offset":20752,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":554,"record_offset":22412,"parent_record_offset":22388,"ancestor_record_offset":20752,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":21332,"parent_record_offset":21308,"ancestor_record_offset":20752,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":21428,"parent_record_offset":21404,"ancestor_record_offset":20752,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":21356,"parent_record_offset":21332,"ancestor_record_offset":20752,"inlinee":11854,"source_line":4043,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::spec_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":554,"record_offset":21380,"parent_record_offset":21356,"ancestor_record_offset":20752,"inlinee":11855,"source_line":372,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::spec_try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":20752,"inlinee":10603,"source_line":4098,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":22552,"parent_record_offset":22388,"ancestor_record_offset":20752,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":23732,"parent_record_offset":23688,"ancestor_record_offset":20752,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":554,"record_offset":23656,"parent_record_offset":23632,"ancestor_record_offset":20752,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":554,"record_offset":24180,"parent_record_offset":23928,"ancestor_record_offset":20752,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":21284,"parent_record_offset":21232,"ancestor_record_offset":20752,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":22180,"parent_record_offset":22156,"ancestor_record_offset":20752,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":20752,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":23056,"parent_record_offset":23024,"ancestor_record_offset":20752,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":23596,"parent_record_offset":23556,"ancestor_record_offset":20752,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":554,"record_offset":24436,"parent_record_offset":24368,"ancestor_record_offset":20752,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":20752,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":22436,"parent_record_offset":22412,"ancestor_record_offset":20752,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":20752,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":23084,"parent_record_offset":23024,"ancestor_record_offset":20752,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":23116,"parent_record_offset":22984,"ancestor_record_offset":20752,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::audio_trigger_library::AudioTriggerCollection::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":554,"record_offset":21208,"parent_record_offset":21184,"ancestor_record_offset":20752,"inlinee":31638,"source_line":23,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":554,"record_offset":21732,"parent_record_offset":21700,"ancestor_record_offset":20752,"inlinee":29930,"source_line":768,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":554,"record_offset":22156,"parent_record_offset":22096,"ancestor_record_offset":20752,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":554,"record_offset":21036,"parent_record_offset":21012,"ancestor_record_offset":20752,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":21856,"parent_record_offset":21828,"ancestor_record_offset":20752,"inlinee":10622,"source_line":813,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":21976,"parent_record_offset":21948,"ancestor_record_offset":20752,"inlinee":10625,"source_line":807,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22004,"parent_record_offset":21976,"ancestor_record_offset":20752,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22096,"parent_record_offset":22004,"ancestor_record_offset":20752,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22068,"parent_record_offset":22004,"ancestor_record_offset":20752,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22128,"parent_record_offset":22096,"ancestor_record_offset":20752,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":22040,"parent_record_offset":22004,"ancestor_record_offset":20752,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":554,"record_offset":21948,"parent_record_offset":21920,"ancestor_record_offset":20752,"inlinee":10624,"source_line":819,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\scopeguard.rs
//     evidence={"module":554,"record_offset":21920,"parent_record_offset":21892,"ancestor_record_offset":20752,"inlinee":6123,"source_line":69,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":554,"record_offset":21232,"parent_record_offset":21208,"ancestor_record_offset":20752,"inlinee":31639,"source_line":497,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::drain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":554,"record_offset":21256,"parent_record_offset":21232,"ancestor_record_offset":20752,"inlinee":31640,"source_line":522,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$10::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":554,"record_offset":21308,"parent_record_offset":21284,"ancestor_record_offset":20752,"inlinee":26908,"source_line":973,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":21012,"parent_record_offset":20988,"ancestor_record_offset":20752,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] utils::index_condenser::IndexCondenser::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/index_condenser.rs
//     evidence={"module":554,"record_offset":21768,"parent_record_offset":21732,"ancestor_record_offset":20752,"inlinee":29931,"source_line":172,"archive":"_pdb/B/modules/00554.jsonl.gz"}
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
// 完整结构及原始类型记录: clearing.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::clearing::PlacementConfigEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::clearing::PlacementConfigEntry>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::clearing::PlacementConfigEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq<country_core::startup::clearing::PlacementConfigEntry,serde_json::de::SeqAccess<serde_json::read::StrRead> >", "constructor": "serde::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "serde_json::de::SeqAccess<serde_json::read::StrRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_str<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >", "constructor": "serde_json::de::from_str", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["country_core::startup::clearing::PlacementConfigEntry", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry>", "constructor": "core::marker::PhantomData", "arguments": ["country_core::startup::clearing::PlacementConfigEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry> >", "constructor": "serde_json::de::impl$9::next_element_seed", "arguments": ["serde_json::read::StrRead", "core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry>", "constructor": "serde::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["country_core::startup::clearing::PlacementConfigEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::StrRead,serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::StrRead", "serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: clearing.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: clearing.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize
// 信息 ID: c4732f4d1ad94eb4a3943480
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry> >; module=1600, inline_record=131620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 7e8962488266fc19b125bdf1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor> @ 0x141670b -> 0xa14620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 3745320aafd2645e02423d27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=224552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 579f7b9356830772bfcc4ae6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=538, inline_record=185004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 53c52af9a701e2cf69b00fe7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=223748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: ac7509f3693b3fc93c8bf949
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>; module=1600, inline_record=229220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing
// 信息 ID: 6bbd23a8f7eae035b85f693c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 43e1929450c1720459b7678c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> > @ 0xa0f704 -> 0xa78aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> > > @ 0x14f982c -> 0xa78aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing::closure$0
// 信息 ID: 1acc3462b708b9361b5a5af1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::clearing::init_clearing", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::clearing::init_clearing::closure_env$0 <- alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$0::spec_extend<country_core::resources::tree_storage::ForestTree,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::placement_data::PlacementDataSsbo>,country_core::startup::clearing::init_clearing::closure_env$0>,alloc::alloc::Global>; module=1585, inline_record=154052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::init_clearing::closure$1
// 信息 ID: 61671089546010a74268c694
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::clearing::init_clearing", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::clearing::init_clearing::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<glam::f32::vec3::Vec3> >,country_core::startup::clearing::init_clearing::closure_env$1>; module=538, inline_record=185260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::uninit_clearing
// 信息 ID: 9ec102051cf10fe47d0b43b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f1571263866702a2d9c5663c: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)> > @ 0xa0fe3f -> 0xa7a5b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)> > > @ 0x1500021 -> 0xa7a5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::clearing::util_add_clearing_shader
// 信息 ID: d3a81a8b478a570e6a6f7921
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::clearing::init_clearing; module=554, inline_record=16856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
