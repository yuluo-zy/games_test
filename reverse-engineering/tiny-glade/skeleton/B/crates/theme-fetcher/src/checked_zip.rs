// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/theme-fetcher/src/checked_zip.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4f8bc0cf2763140d7bf048e8ba1e982ca2ba1c0fd9cfca8c4fa6ffaf56171be1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: checked_zip.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 258, 845, 847, 2166
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.8"},{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.4"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
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
// IPI IDs: 0x3F23
// 首条位置: {"module":258,"inlinee":16163}
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
// IPI IDs: 0x3F3C
// 首条位置: {"module":258,"inlinee":16188}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":258,"record_offset":38132,"parent_record_offset":38104,"ancestor_record_offset":38104,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":38156,"parent_record_offset":38132,"ancestor_record_offset":38104,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":38180,"parent_record_offset":38156,"ancestor_record_offset":38104,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":38104,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F3A
// 首条位置: {"module":258,"inlinee":16186}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":258,"record_offset":37940,"parent_record_offset":37912,"ancestor_record_offset":37912,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":38076,"parent_record_offset":37968,"ancestor_record_offset":37912,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":38156,"parent_record_offset":38132,"ancestor_record_offset":37912,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":38180,"parent_record_offset":38156,"ancestor_record_offset":37912,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":37912,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":258,"record_offset":38132,"parent_record_offset":38104,"ancestor_record_offset":37912,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":38020,"parent_record_offset":37968,"ancestor_record_offset":37912,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":37968,"parent_record_offset":37940,"ancestor_record_offset":37912,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":38044,"parent_record_offset":38020,"ancestor_record_offset":37912,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":258,"record_offset":38104,"parent_record_offset":37968,"ancestor_record_offset":37912,"inlinee":16188,"source_line":9,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD4C1
// 首条位置: {"module":845,"inlinee":54465}
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
// IPI IDs: 0x3F38
// 首条位置: {"module":258,"inlinee":16184}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":258,"record_offset":38388,"parent_record_offset":37264,"ancestor_record_offset":37264,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":39324,"parent_record_offset":37264,"ancestor_record_offset":37264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":258,"record_offset":39292,"parent_record_offset":37264,"ancestor_record_offset":37264,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":258,"record_offset":39384,"parent_record_offset":37264,"ancestor_record_offset":37264,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":258,"record_offset":37324,"parent_record_offset":37264,"ancestor_record_offset":37264,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":258,"record_offset":38416,"parent_record_offset":37264,"ancestor_record_offset":37264,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":39788,"parent_record_offset":39760,"ancestor_record_offset":37264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":39760,"parent_record_offset":39704,"ancestor_record_offset":37264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":39732,"parent_record_offset":39704,"ancestor_record_offset":37264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":39704,"parent_record_offset":39680,"ancestor_record_offset":37264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":39680,"parent_record_offset":39656,"ancestor_record_offset":37264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":38076,"parent_record_offset":37968,"ancestor_record_offset":37264,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":39864,"parent_record_offset":39840,"ancestor_record_offset":37264,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":258,"record_offset":38156,"parent_record_offset":38132,"ancestor_record_offset":37264,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":258,"record_offset":40536,"parent_record_offset":40508,"ancestor_record_offset":37264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":39632,"parent_record_offset":39608,"ancestor_record_offset":37264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":38180,"parent_record_offset":38156,"ancestor_record_offset":37264,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":258,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":37264,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":258,"record_offset":38132,"parent_record_offset":38104,"ancestor_record_offset":37264,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":258,"record_offset":39456,"parent_record_offset":39432,"ancestor_record_offset":37264,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":258,"record_offset":39432,"parent_record_offset":39408,"ancestor_record_offset":37264,"inlinee":6672,"source_line":126,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":258,"record_offset":39544,"parent_record_offset":39520,"ancestor_record_offset":37264,"inlinee":5157,"source_line":126,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":258,"record_offset":37884,"parent_record_offset":37376,"ancestor_record_offset":37264,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":258,"record_offset":38800,"parent_record_offset":38776,"ancestor_record_offset":37264,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":258,"record_offset":39252,"parent_record_offset":39228,"ancestor_record_offset":37264,"inlinee":5769,"source_line":700,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":37600,"parent_record_offset":37464,"ancestor_record_offset":37264,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":38500,"parent_record_offset":38456,"ancestor_record_offset":37264,"inlinee":5761,"source_line":982,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":37464,"parent_record_offset":37376,"ancestor_record_offset":37264,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":37524,"parent_record_offset":37464,"ancestor_record_offset":37264,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":37376,"parent_record_offset":37324,"ancestor_record_offset":37264,"inlinee":6601,"source_line":1963,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":38456,"parent_record_offset":38416,"ancestor_record_offset":37264,"inlinee":6516,"source_line":1996,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":37968,"parent_record_offset":37940,"ancestor_record_offset":37264,"inlinee":6421,"source_line":2151,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$18::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":258,"record_offset":37940,"parent_record_offset":37912,"ancestor_record_offset":37264,"inlinee":6604,"source_line":126,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":37624,"parent_record_offset":37600,"ancestor_record_offset":37264,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":37556,"parent_record_offset":37524,"ancestor_record_offset":37264,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":258,"record_offset":38104,"parent_record_offset":37968,"ancestor_record_offset":37264,"inlinee":16188,"source_line":9,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":258,"record_offset":37912,"parent_record_offset":37884,"ancestor_record_offset":37264,"inlinee":16186,"source_line":9,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F3D
// 首条位置: {"module":258,"inlinee":16189}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":41328,"parent_record_offset":40896,"ancestor_record_offset":40896,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":258,"record_offset":41300,"parent_record_offset":40896,"ancestor_record_offset":40896,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_direct] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":258,"record_offset":40940,"parent_record_offset":40896,"ancestor_record_offset":40896,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":41628,"parent_record_offset":41604,"ancestor_record_offset":40896,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":258,"record_offset":41604,"parent_record_offset":41540,"ancestor_record_offset":40896,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":41572,"parent_record_offset":41540,"ancestor_record_offset":40896,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":41540,"parent_record_offset":41512,"ancestor_record_offset":40896,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":258,"record_offset":41512,"parent_record_offset":41484,"ancestor_record_offset":40896,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":258,"record_offset":41352,"parent_record_offset":41328,"ancestor_record_offset":40896,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":258,"record_offset":42024,"parent_record_offset":41996,"ancestor_record_offset":40896,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":258,"record_offset":41376,"parent_record_offset":41352,"ancestor_record_offset":40896,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":258,"record_offset":41236,"parent_record_offset":40972,"ancestor_record_offset":40896,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":258,"record_offset":41260,"parent_record_offset":41236,"ancestor_record_offset":40896,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":41176,"parent_record_offset":41036,"ancestor_record_offset":40896,"inlinee":5756,"source_line":220,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":41036,"parent_record_offset":40972,"ancestor_record_offset":40896,"inlinee":5752,"source_line":248,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":41100,"parent_record_offset":41036,"ancestor_record_offset":40896,"inlinee":5753,"source_line":212,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/de.rs
//     evidence={"module":258,"record_offset":40972,"parent_record_offset":40940,"ancestor_record_offset":40896,"inlinee":6533,"source_line":1916,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":41200,"parent_record_offset":41176,"ancestor_record_offset":40896,"inlinee":5757,"source_line":513,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/read.rs
//     evidence={"module":258,"record_offset":41132,"parent_record_offset":41100,"ancestor_record_offset":40896,"inlinee":5755,"source_line":502,"archive":"_pdb/B/modules/00258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD4CB
// 首条位置: {"module":845,"inlinee":54475}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11900,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12216,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12156,"parent_record_offset":11868,"ancestor_record_offset":11868,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":11868,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":12020,"parent_record_offset":11996,"ancestor_record_offset":11868,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":845,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":11868,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":845,"record_offset":12096,"parent_record_offset":12068,"ancestor_record_offset":11868,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":11868,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":11868,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11924,"parent_record_offset":11900,"ancestor_record_offset":11868,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12244,"parent_record_offset":12216,"ancestor_record_offset":11868,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":845,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":11868,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":845,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":11868,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00845.jsonl.gz"}
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
// 首条位置: {"module":845,"record_offset":2588,"rva":14229952,"va":5382939072}
// 编译实例 RVAs: 0xd921c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$u8$GT$$u20$as$u20$std..io..copy..BufferedWriterSpec$GT$::buffer_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-227498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e85687fcff","size":5,"soft_fail":false,"target_va":5382712512,"va":5382940005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t142405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8452c0200","size":5,"soft_fail":false,"target_va":5383082784,"va":5382940374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1240
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8134134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e80ae283ff","size":5,"soft_fail":false,"target_va":5374808640,"va":5382942769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8042232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8084985ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5382942963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8020296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8b89e85ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5382944163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7858455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8e91688ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5382939938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=28
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7858321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e86f1788ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5382939820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-235358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8a268fcff","size":5,"soft_fail":false,"target_va":5382704752,"va":5382940105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=5
//     target_aliases=5 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-111907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8dd4afeff","size":5,"soft_fail":false,"target_va":5382827728,"va":5382939630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db7df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t154359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8f75a0200","size":5,"soft_fail":false,"target_va":5383093744,"va":5382939380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419af170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12700656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8f0cbc100","size":5,"soft_fail":false,"target_va":5395640688,"va":5382940027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419b02e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12702865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e891d4c100","size":5,"soft_fail":false,"target_va":5395645152,"va":5382942282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28907713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8c118b901","size":5,"soft_fail":false,"target_va":5411847168,"va":5382939450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8584632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e848027dff","size":5,"soft_fail":false,"target_va":5374355952,"va":5382940579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28598453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8b560b401","size":5,"soft_fail":false,"target_va":5411540963,"va":5382942505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7816990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8e2b888ff","size":5,"soft_fail":false,"target_va":5375124464,"va":5382941449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t153620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e814580200","size":5,"soft_fail":false,"target_va":5383095824,"va":5382942199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6ddf3353abc1c5bf (.llvm.12582498285691889218)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28604304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e89077b401","size":5,"soft_fail":false,"target_va":5411545360,"va":5382941051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28605448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8087cb401","size":5,"soft_fail":false,"target_va":5411546576,"va":5382941123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t149384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e888470200","size":5,"soft_fail":false,"target_va":5383090912,"va":5382941523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t147146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8ca3e0200","size":5,"soft_fail":false,"target_va":5383087600,"va":5382940449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t145957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8253a0200","size":5,"soft_fail":false,"target_va":5383086224,"va":5382940262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28599308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e80c64b401","size":5,"soft_fail":false,"target_va":5411541776,"va":5382942463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$theme_fetcher..checked_zip..FolderChecksumInfo$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8c8e1ffff","size":5,"soft_fail":false,"target_va":5382934640,"va":5382942371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$zip..read..ZipFile$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e80dd5ffff","size":5,"soft_fail":false,"target_va":5382929728,"va":5382940718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28600434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e87268b401","size":5,"soft_fail":false,"target_va":5411542880,"va":5382942441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde::ser::SerializeMap::serialize_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12700530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e872cbc100","size":5,"soft_fail":false,"target_va":5395642368,"va":5382941833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-246300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8e43dfcff","size":5,"soft_fail":false,"target_va":5382694656,"va":5382940951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-230065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e84f7dfcff","size":5,"soft_fail":false,"target_va":5382709584,"va":5382939644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-232212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8ec74fcff","size":5,"soft_fail":false,"target_va":5382708672,"va":5382940879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-233851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8856efcff","size":5,"soft_fail":false,"target_va":5382706224,"va":5382940070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::Write::write_all
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12702118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8a6d1c100","size":5,"soft_fail":false,"target_va":5395644384,"va":5382942261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-195465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e87704fdff","size":5,"soft_fail":false,"target_va":5382744080,"va":5382939540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-198484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8acf8fcff","size":5,"soft_fail":false,"target_va":5382741200,"va":5382939679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-163121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e8cf82fdff","size":5,"soft_fail":false,"target_va":5382776592,"va":5382939708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zip::read::ZipFile::enclosed_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12697874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382939072,"raw_hex":"e812c1c100","size":5,"soft_fail":false,"target_va":5395637376,"va":5382939497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5382939072,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5382939791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":845,"record_offset":6484,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":845,"record_offset":7156,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":7572,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":5284,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$26::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":2876,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":10022,"source_line":3797,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":845,"record_offset":9260,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":845,"record_offset":2932,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":54467,"source_line":1010,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":845,"record_offset":3080,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":845,"record_offset":3208,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":845,"record_offset":6448,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":845,"record_offset":7768,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":845,"record_offset":3176,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":845,"record_offset":9372,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":845,"record_offset":3576,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":8791,"source_line":1391,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11468,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":8157,"source_line":2185,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":8308,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":54474,"source_line":523,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":3788,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":5796,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::io::copy::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\copy.rs
//     evidence={"module":845,"record_offset":5680,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":20255,"source_line":61,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":4212,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":13003,"source_line":3025,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":3236,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":6384,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":5177,"source_line":2250,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":4072,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] zip::read::ZipFile::name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":845,"record_offset":3264,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":20252,"source_line":858,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] zip::read::ZipFile::size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":845,"record_offset":5224,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":54473,"source_line":926,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_direct] zip::read::zip_archive::ZipArchive::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zip","version":"0.6.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zip-0.6.6/src/read.rs
//     evidence={"module":845,"record_offset":3024,"parent_record_offset":2588,"ancestor_record_offset":2588,"inlinee":54472,"source_line":483,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":2588,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":2588,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":2588,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":5576,"parent_record_offset":5400,"ancestor_record_offset":2588,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":845,"record_offset":4732,"parent_record_offset":4644,"ancestor_record_offset":2588,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":845,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":2588,"inlinee":10898,"source_line":335,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":845,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":2588,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":3460,"parent_record_offset":3432,"ancestor_record_offset":2588,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":2588,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":7460,"parent_record_offset":7432,"ancestor_record_offset":2588,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":2588,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":3516,"parent_record_offset":3488,"ancestor_record_offset":2588,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":3488,"parent_record_offset":3460,"ancestor_record_offset":2588,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":5400,"parent_record_offset":5368,"ancestor_record_offset":2588,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":2588,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":2588,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":845,"record_offset":5436,"parent_record_offset":5400,"ancestor_record_offset":2588,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6644,"parent_record_offset":6612,"ancestor_record_offset":2588,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6612,"parent_record_offset":6580,"ancestor_record_offset":2588,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6548,"parent_record_offset":6516,"ancestor_record_offset":2588,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6580,"parent_record_offset":6548,"ancestor_record_offset":2588,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":845,"record_offset":6516,"parent_record_offset":6484,"ancestor_record_offset":2588,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":845,"record_offset":3360,"parent_record_offset":3292,"ancestor_record_offset":2588,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":845,"record_offset":7244,"parent_record_offset":7192,"ancestor_record_offset":2588,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":845,"record_offset":7192,"parent_record_offset":7156,"ancestor_record_offset":2588,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":845,"record_offset":10920,"parent_record_offset":10884,"ancestor_record_offset":2588,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":845,"record_offset":10884,"parent_record_offset":10860,"ancestor_record_offset":2588,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":12068,"parent_record_offset":12044,"ancestor_record_offset":2588,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":7616,"parent_record_offset":7572,"ancestor_record_offset":2588,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":3432,"parent_record_offset":3396,"ancestor_record_offset":2588,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":3396,"parent_record_offset":3360,"ancestor_record_offset":2588,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":12020,"parent_record_offset":11996,"ancestor_record_offset":2588,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":2588,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2588,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":11528,"parent_record_offset":11468,"ancestor_record_offset":2588,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":2588,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":2588,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$30::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":2588,"inlinee":4954,"source_line":3825,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":845,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":2588,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":845,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":2588,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":845,"record_offset":9332,"parent_record_offset":9292,"ancestor_record_offset":2588,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":845,"record_offset":9292,"parent_record_offset":9260,"ancestor_record_offset":2588,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":845,"record_offset":15072,"parent_record_offset":15048,"ancestor_record_offset":2588,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":845,"record_offset":15048,"parent_record_offset":15024,"ancestor_record_offset":2588,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":845,"record_offset":15148,"parent_record_offset":15120,"ancestor_record_offset":2588,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":845,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":2588,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":845,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":2588,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":845,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2588,"inlinee":54470,"source_line":477,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":845,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":2588,"inlinee":54468,"source_line":1001,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":845,"record_offset":4676,"parent_record_offset":4644,"ancestor_record_offset":2588,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":845,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":2588,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":845,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":2588,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":845,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2588,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":845,"record_offset":3688,"parent_record_offset":3660,"ancestor_record_offset":2588,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":845,"record_offset":3840,"parent_record_offset":3788,"ancestor_record_offset":2588,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":845,"record_offset":7272,"parent_record_offset":7192,"ancestor_record_offset":2588,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":845,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":2588,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":845,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":2588,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":845,"record_offset":3108,"parent_record_offset":3080,"ancestor_record_offset":2588,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":845,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":2588,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":845,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":2588,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":845,"record_offset":10732,"parent_record_offset":10704,"ancestor_record_offset":2588,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":845,"record_offset":7076,"parent_record_offset":6644,"ancestor_record_offset":2588,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=89
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":845,"record_offset":4304,"parent_record_offset":4212,"ancestor_record_offset":2588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":845,"record_offset":4444,"parent_record_offset":4416,"ancestor_record_offset":2588,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":845,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":2588,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":845,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":2588,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":845,"record_offset":10952,"parent_record_offset":10920,"ancestor_record_offset":2588,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":845,"record_offset":7736,"parent_record_offset":7572,"ancestor_record_offset":2588,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":845,"record_offset":7300,"parent_record_offset":7192,"ancestor_record_offset":2588,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":845,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":2588,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":845,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":2588,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":845,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":2588,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":845,"record_offset":3660,"parent_record_offset":3632,"ancestor_record_offset":2588,"inlinee":5362,"source_line":2612,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":845,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2588,"inlinee":8777,"source_line":1024,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_suffix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":845,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":2588,"inlinee":20253,"source_line":603,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":845,"record_offset":10980,"parent_record_offset":10920,"ancestor_record_offset":2588,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":845,"record_offset":11012,"parent_record_offset":10884,"ancestor_record_offset":2588,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11948,"parent_record_offset":11924,"ancestor_record_offset":2588,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":2588,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11812,"parent_record_offset":11776,"ancestor_record_offset":2588,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":2588,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11924,"parent_record_offset":11900,"ancestor_record_offset":2588,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11900,"parent_record_offset":11868,"ancestor_record_offset":2588,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12244,"parent_record_offset":12216,"ancestor_record_offset":2588,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12216,"parent_record_offset":11868,"ancestor_record_offset":2588,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":12156,"parent_record_offset":11868,"ancestor_record_offset":2588,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":845,"record_offset":11776,"parent_record_offset":11468,"ancestor_record_offset":2588,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":845,"record_offset":6412,"parent_record_offset":6384,"ancestor_record_offset":2588,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":845,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":2588,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":845,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":2588,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":845,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":2588,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":4036,"parent_record_offset":3788,"ancestor_record_offset":2588,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":2588,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":8340,"parent_record_offset":8308,"ancestor_record_offset":2588,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":8600,"parent_record_offset":8308,"ancestor_record_offset":2588,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":845,"record_offset":4244,"parent_record_offset":4212,"ancestor_record_offset":2588,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Read::read_to_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":845,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":2588,"inlinee":23007,"source_line":881,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::generic_copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\copy.rs
//     evidence={"module":845,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":2588,"inlinee":21294,"source_line":77,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::copy::impl$6::copy_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\copy.rs
//     evidence={"module":845,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":2588,"inlinee":22908,"source_line":263,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":845,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":2588,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":845,"record_offset":4388,"parent_record_offset":4360,"ancestor_record_offset":2588,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":845,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":2588,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":845,"record_offset":11972,"parent_record_offset":11948,"ancestor_record_offset":2588,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":845,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":2588,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":2588,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":3888,"parent_record_offset":3864,"ancestor_record_offset":2588,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":845,"record_offset":3864,"parent_record_offset":3840,"ancestor_record_offset":2588,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":845,"record_offset":8400,"parent_record_offset":8340,"ancestor_record_offset":2588,"inlinee":32384,"source_line":221,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":845,"record_offset":8372,"parent_record_offset":8340,"ancestor_record_offset":2588,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/00845.jsonl.gz"}
//   [pdb_inline_transitive] theme_fetcher::checked_zip::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"theme-fetcher","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/theme-fetcher/src/checked_zip.rs
//     evidence={"module":845,"record_offset":11868,"parent_record_offset":11776,"ancestor_record_offset":2588,"inlinee":54475,"source_line":9,"archive":"_pdb/B/modules/00845.jsonl.gz"}
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
// 首条位置: {"module":847,"record_offset":2272,"rva":14236064,"va":5382945184}
// 编译实例 RVAs: 0xd939a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$blake3..Hash$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t136773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e845160200","size":5,"soft_fail":false,"target_va":5383082784,"va":5382946006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9406044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e85c868f00","size":5,"soft_fail":false,"target_va":5392353392,"va":5382947343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1240
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8140219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e845ca83ff","size":5,"soft_fail":false,"target_va":5374808640,"va":5382948854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8047342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8123585ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5382948073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8044278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e80a4185ff","size":5,"soft_fail":false,"target_va":5374904480,"va":5382948753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=36
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7863809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8ff0188ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5382945308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7863913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8970188ff","size":5,"soft_fail":false,"target_va":5375081568,"va":5382945476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-117543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8d934feff","size":5,"soft_fail":false,"target_va":5382827728,"va":5382945266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d8f4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8ddabffff","size":5,"soft_fail":false,"target_va":5382927584,"va":5382949118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419b0180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12698151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e827c2c100","size":5,"soft_fail":false,"target_va":5395644800,"va":5382946644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: checked_zip.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28901088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8e0feb801","size":5,"soft_fail":false,"target_va":5411847152,"va":5382946059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-107487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8215cfeff","size":5,"soft_fail":false,"target_va":5382840208,"va":5382947690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28381137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8d10fb101","size":5,"soft_fail":false,"target_va":5411327296,"va":5382946154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h6ddf3353abc1c5bf (.llvm.12582498285691889218)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28599923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e87366b401","size":5,"soft_fail":false,"target_va":5411545360,"va":5382945432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::he1efa0065b165f30 (.llvm.12582498285691889218)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28600747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8ab69b401","size":5,"soft_fail":false,"target_va":5411546352,"va":5382945600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28596836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8645ab401","size":5,"soft_fail":false,"target_va":5411544576,"va":5382947735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t28598836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e83462b401","size":5,"soft_fail":false,"target_va":5411546880,"va":5382948039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t145021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e87d360200","size":5,"soft_fail":false,"target_va":5383090912,"va":5382945886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t141764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8c4290200","size":5,"soft_fail":false,"target_va":5383087600,"va":5382945831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t139303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e827200200","size":5,"soft_fail":false,"target_va":5383086224,"va":5382946916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<memmap2::Mmap,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8b3edffff","size":5,"soft_fail":false,"target_va":5382944800,"va":5382949480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28594486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e83651b401","size":5,"soft_fail":false,"target_va":5411542880,"va":5382948389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9407217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8f18a8f00","size":5,"soft_fail":false,"target_va":5392353952,"va":5382946730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2101363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e88defdfff","size":5,"soft_fail":false,"target_va":5380845328,"va":5382946686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-250702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8b22cfcff","size":5,"soft_fail":false,"target_va":5382694656,"va":5382945353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-237962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8765efcff","size":5,"soft_fail":false,"target_va":5382708672,"va":5382946629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-240473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e8a754fcff","size":5,"soft_fail":false,"target_va":5382704816,"va":5382945284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-201183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382945184,"raw_hex":"e821eefcff","size":5,"soft_fail":false,"target_va":5382744080,"va":5382945258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*37741582(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382945184,"raw_hex":"ff150ee43f02","size":6,"soft_fail":false,"va":5382946780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":847,"record_offset":9700,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":847,"record_offset":3948,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":847,"record_offset":3648,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":6240,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":847,"record_offset":3308,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":847,"record_offset":3556,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":54467,"source_line":1010,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":847,"record_offset":4364,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":847,"record_offset":4756,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":847,"record_offset":6772,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":847,"record_offset":4528,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":9444,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":8221,"source_line":709,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":6968,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":11464,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":847,"record_offset":3796,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":847,"record_offset":6528,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":13581,"source_line":579,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":847,"record_offset":6412,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":847,"record_offset":2504,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":847,"record_offset":2476,"parent_record_offset":2272,"ancestor_record_offset":2272,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":847,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":2272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":847,"record_offset":2904,"parent_record_offset":2852,"ancestor_record_offset":2272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":847,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":2272,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":847,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":2272,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":847,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":847,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":847,"record_offset":2828,"parent_record_offset":2808,"ancestor_record_offset":2272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":847,"record_offset":4036,"parent_record_offset":3984,"ancestor_record_offset":2272,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":847,"record_offset":3984,"parent_record_offset":3948,"ancestor_record_offset":2272,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":847,"record_offset":4392,"parent_record_offset":4364,"ancestor_record_offset":2272,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":847,"record_offset":3704,"parent_record_offset":3672,"ancestor_record_offset":2272,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":847,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":2272,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":847,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":2272,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":10556,"parent_record_offset":9980,"ancestor_record_offset":2272,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":2272,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":2272,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":847,"record_offset":10628,"parent_record_offset":10604,"ancestor_record_offset":2272,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":847,"record_offset":4420,"parent_record_offset":4392,"ancestor_record_offset":2272,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":847,"record_offset":3396,"parent_record_offset":3348,"ancestor_record_offset":2272,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":847,"record_offset":3348,"parent_record_offset":3308,"ancestor_record_offset":2272,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":847,"record_offset":13532,"parent_record_offset":13508,"ancestor_record_offset":2272,"inlinee":6205,"source_line":253,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":847,"record_offset":13508,"parent_record_offset":13484,"ancestor_record_offset":2272,"inlinee":6203,"source_line":49,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":847,"record_offset":13608,"parent_record_offset":13580,"ancestor_record_offset":2272,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":847,"record_offset":13556,"parent_record_offset":13532,"ancestor_record_offset":2272,"inlinee":6207,"source_line":72,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":847,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":2272,"inlinee":6208,"source_line":76,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":847,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":2272,"inlinee":54470,"source_line":477,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":847,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":2272,"inlinee":54468,"source_line":1001,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":847,"record_offset":8332,"parent_record_offset":8300,"ancestor_record_offset":2272,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":847,"record_offset":4064,"parent_record_offset":3984,"ancestor_record_offset":2272,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":847,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":2272,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":847,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":2272,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":847,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":2272,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":847,"record_offset":5480,"parent_record_offset":5452,"ancestor_record_offset":2272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":847,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":2272,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":847,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":2272,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":847,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":2272,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=103
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":847,"record_offset":2708,"parent_record_offset":2504,"ancestor_record_offset":2272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":2272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":2272,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":8100,"parent_record_offset":8072,"ancestor_record_offset":2272,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":2272,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":847,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":2272,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":847,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":2272,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":847,"record_offset":6344,"parent_record_offset":6240,"ancestor_record_offset":2272,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":847,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":2272,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":847,"record_offset":3832,"parent_record_offset":3796,"ancestor_record_offset":2272,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":847,"record_offset":6372,"parent_record_offset":6344,"ancestor_record_offset":2272,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":4092,"parent_record_offset":3984,"ancestor_record_offset":2272,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":6728,"parent_record_offset":6556,"ancestor_record_offset":2272,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":2272,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":847,"record_offset":6700,"parent_record_offset":6556,"ancestor_record_offset":2272,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":847,"record_offset":4448,"parent_record_offset":4420,"ancestor_record_offset":2272,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":847,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":2272,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":847,"record_offset":3732,"parent_record_offset":3672,"ancestor_record_offset":2272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":847,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":2272,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":847,"record_offset":3860,"parent_record_offset":3796,"ancestor_record_offset":2272,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":847,"record_offset":6588,"parent_record_offset":6556,"ancestor_record_offset":2272,"inlinee":13584,"source_line":228,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":847,"record_offset":6556,"parent_record_offset":6528,"ancestor_record_offset":2272,"inlinee":13583,"source_line":338,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":847,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":2272,"inlinee":13586,"source_line":229,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":847,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":2272,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":847,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":2272,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":847,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2272,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":847,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":2272,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":847,"record_offset":6496,"parent_record_offset":6412,"ancestor_record_offset":2272,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":847,"record_offset":8072,"parent_record_offset":8040,"ancestor_record_offset":2272,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":847,"record_offset":8040,"parent_record_offset":8012,"ancestor_record_offset":2272,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":847,"record_offset":6892,"parent_record_offset":6868,"ancestor_record_offset":2272,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":847,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":2272,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":847,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":2272,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":847,"record_offset":2540,"parent_record_offset":2504,"ancestor_record_offset":2272,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":847,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":2272,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/00847.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: theme_fetcher::checked_zip::verify_folder_checksums::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16840
// 首条位置: {"module":2166,"inlinee":92224}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2166,"record_offset":532,"parent_record_offset":512,"ancestor_record_offset":512,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02166.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2166,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":512,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02166.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2166,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":512,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02166.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_slice<theme_fetcher::checked_zip::FolderChecksumInfo>", "constructor": "serde_json::de::from_slice", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<theme_fetcher::checked_zip::FolderChecksumInfo,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<theme_fetcher::checked_zip::FolderChecksumInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["theme_fetcher::checked_zip::FolderChecksumInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize
// 信息 ID: cc2ab528e92835bfb5a12e9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::from_slice<theme_fetcher::checked_zip::FolderChecksumInfo>; module=258, inline_record=544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: e0ba2f85ad3371b297a8edd1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=258, inline_record=38104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 8bf6b33d03d8399de3921c74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=258, inline_record=37912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 3e30a9fcbd7d2a9df105ca80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=845, inline_record=432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 9bb65e10069d236ac6cb3ddc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=258, inline_record=37264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: b102cd6b95f4746861912ec1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::SliceRead,theme_fetcher::checked_zip::_::impl$0::deserialize::__Visitor>; module=258, inline_record=40896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::_::impl$0::serialize
// 信息 ID: f3cc4d2be894e4d298f2a650
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >; module=845, inline_record=11868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::extract_with_checksums<std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> > >
// 信息 ID: 2f230566b7c03de394947fa7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["bytes::bytes::Bytes"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["ref$<bytes::bytes::Bytes>"]; 字段布局 = unknown。
// [类型关系] theme_fetcher::checked_zip::extract_with_checksums -> ["std::io::cursor::Cursor<ref$<bytes::bytes::Bytes> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x61b42a -> 0xd921c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::verify_folder_checksums
// 信息 ID: 96c525f6fcbd7e83db80fbd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] theme_fetcher::ThemeFetcher::run @ 0x618ade -> 0xd939a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: theme_fetcher::checked_zip::verify_folder_checksums::closure$0
// 信息 ID: f29d987cae0a88b34bb8f577
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "theme_fetcher::checked_zip::verify_folder_checksums", "captures": "unknown"}
// [闭包环境类型引用] theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,theme_fetcher::checked_zip::verify_folder_checksums::closure_env$0>; module=2166, inline_record=512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
