// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tg-text/src/font_table.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f574bf591a470045720fef15e96d70042a98770478f4ffae939d945490e3e9a2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: font_table.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 335, 336, 624, 625, 1182, 1195, 1205, 1253, 1779, 2778, 2804, 3942
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
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
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 5, 16, 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2A4
// 首条位置: {"module":1205,"inlinee":62116}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":2778,"record_offset":75880,"parent_record_offset":75856,"ancestor_record_offset":75856,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":2778,"record_offset":46560,"parent_record_offset":46536,"ancestor_record_offset":46536,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 5, 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186EC
// 首条位置: {"module":2778,"inlinee":100076}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2778,"record_offset":70248,"parent_record_offset":70224,"ancestor_record_offset":70224,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2778,"record_offset":70272,"parent_record_offset":70248,"ancestor_record_offset":70224,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":70296,"parent_record_offset":70272,"ancestor_record_offset":70224,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":70320,"parent_record_offset":70296,"ancestor_record_offset":70224,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 5, 16, 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186E1
// 首条位置: {"module":2778,"inlinee":100065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":12008,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2778,"record_offset":70272,"parent_record_offset":70248,"ancestor_record_offset":70100,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":12068,"parent_record_offset":12032,"ancestor_record_offset":12008,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":12132,"parent_record_offset":12032,"ancestor_record_offset":12008,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":70296,"parent_record_offset":70272,"ancestor_record_offset":70100,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":70320,"parent_record_offset":70296,"ancestor_record_offset":70100,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2778,"record_offset":70248,"parent_record_offset":70224,"ancestor_record_offset":70100,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":12104,"parent_record_offset":12032,"ancestor_record_offset":12008,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2778,"record_offset":70224,"parent_record_offset":70200,"ancestor_record_offset":70100,"inlinee":100076,"source_line":5,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5, 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186EB
// 首条位置: {"module":2778,"inlinee":100075}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2778,"record_offset":70520,"parent_record_offset":69464,"ancestor_record_offset":69464,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2778,"record_offset":71260,"parent_record_offset":69464,"ancestor_record_offset":69464,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":70420,"parent_record_offset":69464,"ancestor_record_offset":69464,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":69516,"parent_record_offset":69464,"ancestor_record_offset":69464,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":70456,"parent_record_offset":69464,"ancestor_record_offset":69464,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2778,"record_offset":89556,"parent_record_offset":89532,"ancestor_record_offset":87664,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2778,"record_offset":89532,"parent_record_offset":89460,"ancestor_record_offset":87664,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2778,"record_offset":89496,"parent_record_offset":89460,"ancestor_record_offset":87664,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2778,"record_offset":89460,"parent_record_offset":89428,"ancestor_record_offset":87664,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2778,"record_offset":89428,"parent_record_offset":89400,"ancestor_record_offset":87664,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2778,"record_offset":70272,"parent_record_offset":70248,"ancestor_record_offset":69464,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2778,"record_offset":69728,"parent_record_offset":69548,"ancestor_record_offset":69464,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2778,"record_offset":69856,"parent_record_offset":69548,"ancestor_record_offset":69464,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2778,"record_offset":70984,"parent_record_offset":70584,"ancestor_record_offset":69464,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":69692,"parent_record_offset":69548,"ancestor_record_offset":69464,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":70296,"parent_record_offset":70272,"ancestor_record_offset":69464,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2778,"record_offset":70320,"parent_record_offset":70296,"ancestor_record_offset":69464,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2778,"record_offset":70248,"parent_record_offset":70224,"ancestor_record_offset":69464,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2778,"record_offset":71056,"parent_record_offset":71032,"ancestor_record_offset":69464,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":2778,"record_offset":69760,"parent_record_offset":69548,"ancestor_record_offset":69464,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_f32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":89144,"parent_record_offset":89120,"ancestor_record_offset":87664,"inlinee":5594,"source_line":424,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":70128,"parent_record_offset":70100,"ancestor_record_offset":69464,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":69548,"parent_record_offset":69516,"ancestor_record_offset":69464,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":2778,"record_offset":70584,"parent_record_offset":70548,"ancestor_record_offset":69464,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_f32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":2778,"record_offset":89120,"parent_record_offset":89096,"ancestor_record_offset":87664,"inlinee":5593,"source_line":118,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":2778,"record_offset":70816,"parent_record_offset":70792,"ancestor_record_offset":69464,"inlinee":100077,"source_line":214,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":70200,"parent_record_offset":70128,"ancestor_record_offset":69464,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":69800,"parent_record_offset":69548,"ancestor_record_offset":69464,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":2778,"record_offset":70792,"parent_record_offset":70768,"ancestor_record_offset":69464,"inlinee":73646,"source_line":1505,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":2778,"record_offset":89096,"parent_record_offset":89072,"ancestor_record_offset":87664,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2778,"record_offset":70224,"parent_record_offset":70200,"ancestor_record_offset":69464,"inlinee":100076,"source_line":5,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2778,"record_offset":69824,"parent_record_offset":69800,"ancestor_record_offset":69464,"inlinee":100065,"source_line":5,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::default_scale
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":625,"record_offset":256,"rva":11560832,"va":5380269952}
// 编译实例 RVAs: 0xb06780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4DF4
// 首条位置: {"module":335,"inlinee":19956}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":335,"record_offset":48708,"parent_record_offset":48680,"ancestor_record_offset":48680,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":335,"record_offset":48908,"parent_record_offset":48880,"ancestor_record_offset":48680,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":335,"record_offset":48960,"parent_record_offset":48936,"ancestor_record_offset":48680,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":335,"record_offset":48936,"parent_record_offset":48880,"ancestor_record_offset":48680,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":335,"record_offset":48984,"parent_record_offset":48960,"ancestor_record_offset":48680,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":335,"record_offset":48852,"parent_record_offset":48824,"ancestor_record_offset":48680,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":335,"record_offset":49008,"parent_record_offset":48984,"ancestor_record_offset":48680,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":335,"record_offset":48764,"parent_record_offset":48736,"ancestor_record_offset":48680,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":335,"record_offset":48880,"parent_record_offset":48824,"ancestor_record_offset":48680,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":335,"record_offset":48736,"parent_record_offset":48708,"ancestor_record_offset":48680,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":335,"record_offset":48824,"parent_record_offset":48792,"ancestor_record_offset":48680,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":335,"record_offset":48792,"parent_record_offset":48764,"ancestor_record_offset":48680,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00335.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::FontTable::get
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x868C
// 首条位置: {"module":624,"inlinee":34444}
// 编译实例 RVAs: 0xb91fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1790
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: font_table.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6162491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380841376,"raw_hex":"e8c5f7a1ff","size":5,"soft_fail":false,"target_va":5374678928,"va":5380841414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":624,"record_offset":23564,"parent_record_offset":23540,"ancestor_record_offset":23540,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":624,"record_offset":24340,"parent_record_offset":24316,"ancestor_record_offset":23540,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":624,"record_offset":23848,"parent_record_offset":23824,"ancestor_record_offset":23540,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":624,"record_offset":23932,"parent_record_offset":23908,"ancestor_record_offset":23540,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":624,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":23540,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":624,"record_offset":24408,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":624,"record_offset":24080,"parent_record_offset":24052,"ancestor_record_offset":23540,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":624,"record_offset":24052,"parent_record_offset":24024,"ancestor_record_offset":23540,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":624,"record_offset":24200,"parent_record_offset":24176,"ancestor_record_offset":23540,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":624,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":23540,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":625,"record_offset":924,"parent_record_offset":896,"ancestor_record_offset":368,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":624,"record_offset":24024,"parent_record_offset":23992,"ancestor_record_offset":23540,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":625,"record_offset":956,"parent_record_offset":924,"ancestor_record_offset":368,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":624,"record_offset":23964,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":625,"record_offset":896,"parent_record_offset":648,"ancestor_record_offset":368,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":624,"record_offset":23824,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":625,"record_offset":756,"parent_record_offset":648,"ancestor_record_offset":368,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":624,"record_offset":24436,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":625,"record_offset":1364,"parent_record_offset":648,"ancestor_record_offset":368,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":624,"record_offset":23908,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":625,"record_offset":840,"parent_record_offset":648,"ancestor_record_offset":368,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":624,"record_offset":23796,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":624,"record_offset":24316,"parent_record_offset":24292,"ancestor_record_offset":23540,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":624,"record_offset":23588,"parent_record_offset":23564,"ancestor_record_offset":23540,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":625,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":368,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":624,"record_offset":23612,"parent_record_offset":23588,"ancestor_record_offset":23540,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":625,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":368,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":624,"record_offset":24292,"parent_record_offset":24124,"ancestor_record_offset":23540,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":625,"record_offset":1220,"parent_record_offset":1056,"ancestor_record_offset":368,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":624,"record_offset":23640,"parent_record_offset":23612,"ancestor_record_offset":23540,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":624,"record_offset":24232,"parent_record_offset":24148,"ancestor_record_offset":23540,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":624,"record_offset":24176,"parent_record_offset":24148,"ancestor_record_offset":23540,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":625,"record_offset":1104,"parent_record_offset":1076,"ancestor_record_offset":368,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":624,"record_offset":24528,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":625,"record_offset":1456,"parent_record_offset":648,"ancestor_record_offset":368,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":624,"record_offset":24148,"parent_record_offset":24124,"ancestor_record_offset":23540,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":625,"record_offset":1076,"parent_record_offset":1056,"ancestor_record_offset":368,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":624,"record_offset":23692,"parent_record_offset":23668,"ancestor_record_offset":23540,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":625,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":368,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":624,"record_offset":23668,"parent_record_offset":23612,"ancestor_record_offset":23540,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":625,"record_offset":596,"parent_record_offset":536,"ancestor_record_offset":368,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":624,"record_offset":23716,"parent_record_offset":23692,"ancestor_record_offset":23540,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":625,"record_offset":648,"parent_record_offset":620,"ancestor_record_offset":368,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":624,"record_offset":23768,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":625,"record_offset":700,"parent_record_offset":648,"ancestor_record_offset":368,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":624,"record_offset":24124,"parent_record_offset":23716,"ancestor_record_offset":23540,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":625,"record_offset":1056,"parent_record_offset":648,"ancestor_record_offset":368,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00625.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::impl$6::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":624,"record_offset":24364,"parent_record_offset":24340,"ancestor_record_offset":23540,"inlinee":34442,"source_line":27,"archive":"_pdb/B/modules/00624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2804,"record_offset":256,"rva":31839120,"va":5400548240}
// 编译实例 RVAs: 0x1e5d390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10932741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400548240,"raw_hex":"e805d2a600","size":5,"soft_fail":false,"target_va":5411481152,"va":5400548406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2804,"record_offset":432,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02804.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2804,"record_offset":460,"parent_record_offset":432,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2804,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":256,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02804.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2804,"record_offset":516,"parent_record_offset":488,"ancestor_record_offset":256,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x186EA
// 首条位置: {"module":2778,"inlinee":100074}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2778,"record_offset":60096,"parent_record_offset":60040,"ancestor_record_offset":60040,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":2778,"record_offset":60068,"parent_record_offset":60040,"ancestor_record_offset":60040,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/02778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x868A
// 首条位置: {"module":624,"inlinee":34442}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::impl$8::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1B3
// 首条位置: {"module":1182,"inlinee":61875}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1182,"record_offset":600,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01182.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1182,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":580,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01182.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1182,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":580,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01182.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1182,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":580,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01182.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1182,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":580,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01182.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1182,"record_offset":700,"parent_record_offset":648,"ancestor_record_offset":580,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01182.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::FontTableEntry::scaled
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x868E
// 首条位置: {"module":624,"inlinee":34446}
// 编译实例 RVAs: 0xb92090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: font_table.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::font_table::FontFamily
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "ref$<tg_text::font_table::FontFamily>", "constructor": "ref$", "arguments": ["tg_text::font_table::FontFamily"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<tg_text::font_table::FontFamily>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "constructor": "tuple$", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map::impl$0::fmt<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map::impl$0::fmt", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::font_table::FontTable
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<tg_text::font_table::FontTable,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["tg_text::font_table::FontTable", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<tg_text::font_table::FontTable>", "constructor": "core::marker::PhantomData", "arguments": ["tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_str_seed<core::marker::PhantomData<tg_text::font_table::FontTable>,tg_text::font_table::FontTable>", "constructor": "ron::options::Options::from_str_seed", "arguments": ["core::marker::PhantomData<tg_text::font_table::FontTable>", "tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::font_table::FontTableEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "constructor": "tuple$", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map::impl$0::fmt<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map::impl$0::fmt", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: font_table.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: font_table.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>

// ------------------------------------------------------------------
// 函数: tg_text::font_table::FontTable::get
// 信息 ID: e2b63730c0f935a0adc209b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x10639a -> 0xb91fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0xfac19a -> 0xb91fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::new<f32> @ 0xfac24e -> 0xb91fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x25f0b0a -> 0xb91fa0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=624, inline_record=23540; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::FontTableEntry::scaled
// 信息 ID: 84710de9c9802bcd2a99c292
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x1063b1 -> 0xb92090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0xfac1b1 -> 0xb92090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::new<f32> @ 0xfac26d -> 0xb92090 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x25f0b21 -> 0xb92090 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=624, inline_record=24588; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize
// 信息 ID: 7f23c3112021170f45b760f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<tg_text::font_table::FontTable>,tg_text::font_table::FontTable>; module=1253, inline_record=1076; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >; module=2778, inline_record=46228; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >; module=2778, inline_record=46536; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<toolbar::mei::config::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=75856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 2182a10582d3339e3ae82b37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=70224; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=88464; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=3942, inline_record=5280; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=3942, inline_record=5704; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=3942, inline_record=6448; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=3942, inline_record=6724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 6c8cfab221b5dbfcc34ee88f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::font_table::_::impl$0::deserialize::__Field> > @ 0x1e3ac96 -> 0x1e5d390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 0630082cae9af702cf187659
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::font_table::_::impl$0::deserialize::__Field> >; module=2778, inline_record=12008; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=69824; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=69980; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=70100; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=88044; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=88200; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=88324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 7e9f0780986d4c8baca6bd28
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=60040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: daa5431ac77913256ed1742b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=69464; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2778, inline_record=87664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::default_scale
// 信息 ID: 49e1ba96e59bfb211947536c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor> @ 0x1e4af9a -> 0xb06780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::show_color_menu::show_color_options @ 0x22a0a09 -> 0xb06780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::show_color_menu::show_color_options @ 0x229ff26 -> 0xb06780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::show_color_menu::show_color_options @ 0x229fabf -> 0xb06780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::show_color_menu::show_color_options @ 0x229f57d -> 0xb06780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$2::default
// 信息 ID: f52debc29fda330d3e68daac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::impl$23::build; module=335, inline_record=48680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$6::eq
// 信息 ID: eb5fdb71774d8698295fac52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=624, inline_record=23424; 不是独立调用边。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=624, inline_record=24364; 不是独立调用边。
// [PDB 内联宿主] tg_text::font_table::FontTable::get; module=625, inline_record=1292; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>; module=1195, inline_record=13956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$8::hash
// 信息 ID: 631bcca23ae6f189ba34c5f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >; module=1182, inline_record=580; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >; module=1779, inline_record=2384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
