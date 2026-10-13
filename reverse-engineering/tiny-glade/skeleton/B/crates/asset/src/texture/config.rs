// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset/src/texture/config.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5976cdaf1961319bf2426b560aefb691fd9ef5b98f8082bb544a299615054540
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: config.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3167, 4094, 4141, 4589, 4591
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
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
// 函数符号: asset::texture::config::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7, 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E855
// 首条位置: {"module":4094,"inlinee":125013}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":11116,"parent_record_offset":11096,"ancestor_record_offset":11096,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BBC
// 首条位置: {"module":4589,"inlinee":134076}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4589,"record_offset":35852,"parent_record_offset":35820,"ancestor_record_offset":35820,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":35880,"parent_record_offset":35852,"ancestor_record_offset":35820,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":35908,"parent_record_offset":35880,"ancestor_record_offset":35820,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":35936,"parent_record_offset":35908,"ancestor_record_offset":35820,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 7, 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BB1
// 首条位置: {"module":4589,"inlinee":134065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32368,"parent_record_offset":32340,"ancestor_record_offset":32340,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":35820,"parent_record_offset":35788,"ancestor_record_offset":35676,"inlinee":134076,"source_line":7,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":35880,"parent_record_offset":35852,"ancestor_record_offset":35676,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":32404,"parent_record_offset":32368,"ancestor_record_offset":32340,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":32468,"parent_record_offset":32368,"ancestor_record_offset":32340,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":35908,"parent_record_offset":35880,"ancestor_record_offset":35676,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":35936,"parent_record_offset":35908,"ancestor_record_offset":35676,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4589,"record_offset":35852,"parent_record_offset":35820,"ancestor_record_offset":35676,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32440,"parent_record_offset":32368,"ancestor_record_offset":32340,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7, 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BAF
// 首条位置: {"module":4589,"inlinee":134063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4589,"record_offset":32624,"parent_record_offset":31672,"ancestor_record_offset":31672,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":37100,"parent_record_offset":34980,"ancestor_record_offset":34980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":32584,"parent_record_offset":31672,"ancestor_record_offset":31672,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":37240,"parent_record_offset":34980,"ancestor_record_offset":34980,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":31728,"parent_record_offset":31672,"ancestor_record_offset":31672,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32652,"parent_record_offset":31672,"ancestor_record_offset":31672,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4589,"record_offset":37268,"parent_record_offset":34980,"ancestor_record_offset":34980,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":33252,"parent_record_offset":33228,"ancestor_record_offset":31672,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":35820,"parent_record_offset":35788,"ancestor_record_offset":34980,"inlinee":134076,"source_line":7,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":31672,"inlinee":134065,"source_line":7,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":32876,"parent_record_offset":32852,"ancestor_record_offset":31672,"inlinee":134067,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":34092,"parent_record_offset":34064,"ancestor_record_offset":31672,"inlinee":134075,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":33228,"parent_record_offset":33204,"ancestor_record_offset":31672,"inlinee":134069,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/config.rs
//     evidence={"module":4589,"record_offset":33684,"parent_record_offset":33656,"ancestor_record_offset":31672,"inlinee":134073,"source_line":17,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4589,"record_offset":33276,"parent_record_offset":33252,"ancestor_record_offset":31672,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":4589,"record_offset":37816,"parent_record_offset":37784,"ancestor_record_offset":34980,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":4589,"record_offset":37876,"parent_record_offset":37844,"ancestor_record_offset":34980,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4589,"record_offset":35880,"parent_record_offset":35852,"ancestor_record_offset":34980,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":4589,"record_offset":37672,"parent_record_offset":37648,"ancestor_record_offset":34980,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":31960,"parent_record_offset":31768,"ancestor_record_offset":31672,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":32088,"parent_record_offset":31768,"ancestor_record_offset":31672,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4589,"record_offset":37844,"parent_record_offset":37784,"ancestor_record_offset":34980,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":4589,"record_offset":37420,"parent_record_offset":37380,"ancestor_record_offset":34980,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4589,"record_offset":37536,"parent_record_offset":37508,"ancestor_record_offset":34980,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":36824,"parent_record_offset":36424,"ancestor_record_offset":34980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4589,"record_offset":37624,"parent_record_offset":37508,"ancestor_record_offset":34980,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":37700,"parent_record_offset":37672,"ancestor_record_offset":34980,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":37648,"parent_record_offset":37624,"ancestor_record_offset":34980,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4589,"record_offset":37728,"parent_record_offset":37672,"ancestor_record_offset":34980,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":32508,"parent_record_offset":31768,"ancestor_record_offset":31672,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":31920,"parent_record_offset":31768,"ancestor_record_offset":31672,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":32468,"parent_record_offset":32368,"ancestor_record_offset":31672,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":35908,"parent_record_offset":35880,"ancestor_record_offset":34980,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4589,"record_offset":35936,"parent_record_offset":35908,"ancestor_record_offset":34980,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4589,"record_offset":35852,"parent_record_offset":35820,"ancestor_record_offset":34980,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4589,"record_offset":36896,"parent_record_offset":36872,"ancestor_record_offset":34980,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4589,"record_offset":31992,"parent_record_offset":31768,"ancestor_record_offset":31672,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32368,"parent_record_offset":32340,"ancestor_record_offset":31672,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":31768,"parent_record_offset":31728,"ancestor_record_offset":31672,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32676,"parent_record_offset":32652,"ancestor_record_offset":31672,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::Deserializer::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":32824,"parent_record_offset":32676,"ancestor_record_offset":31672,"inlinee":100071,"source_line":10,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":33300,"parent_record_offset":33276,"ancestor_record_offset":31672,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":36656,"parent_record_offset":36632,"ancestor_record_offset":34980,"inlinee":100077,"source_line":214,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32440,"parent_record_offset":32368,"ancestor_record_offset":31672,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4589,"record_offset":32032,"parent_record_offset":31768,"ancestor_record_offset":31672,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4589,"record_offset":36632,"parent_record_offset":36608,"ancestor_record_offset":34980,"inlinee":73646,"source_line":1505,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4589,"record_offset":37984,"parent_record_offset":37268,"ancestor_record_offset":34980,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":4589,"record_offset":37324,"parent_record_offset":37296,"ancestor_record_offset":34980,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":4589,"record_offset":37584,"parent_record_offset":37560,"ancestor_record_offset":34980,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":4589,"record_offset":37448,"parent_record_offset":37420,"ancestor_record_offset":34980,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":4589,"record_offset":37784,"parent_record_offset":37380,"ancestor_record_offset":34980,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":4589,"record_offset":37296,"parent_record_offset":37268,"ancestor_record_offset":34980,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":4589,"record_offset":37476,"parent_record_offset":37448,"ancestor_record_offset":34980,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":4589,"record_offset":37508,"parent_record_offset":37476,"ancestor_record_offset":34980,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":4589,"record_offset":37380,"parent_record_offset":37352,"ancestor_record_offset":34980,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":4589,"record_offset":37352,"parent_record_offset":37324,"ancestor_record_offset":34980,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A665
// 首条位置: {"module":3167,"inlinee":108133}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3167,"record_offset":4868,"parent_record_offset":4836,"ancestor_record_offset":4836,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3167,"record_offset":5416,"parent_record_offset":5384,"ancestor_record_offset":4836,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3167,"record_offset":5476,"parent_record_offset":5444,"ancestor_record_offset":4836,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3167,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":4836,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3167,"record_offset":5444,"parent_record_offset":5384,"ancestor_record_offset":4836,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3167,"record_offset":5020,"parent_record_offset":4980,"ancestor_record_offset":4836,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":4836,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3167,"record_offset":5224,"parent_record_offset":5108,"ancestor_record_offset":4836,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5300,"parent_record_offset":5272,"ancestor_record_offset":4836,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":4836,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3167,"record_offset":5328,"parent_record_offset":5272,"ancestor_record_offset":4836,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":4924,"parent_record_offset":4896,"ancestor_record_offset":4836,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3167,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":4836,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3167,"record_offset":5048,"parent_record_offset":5020,"ancestor_record_offset":4836,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":5384,"parent_record_offset":4980,"ancestor_record_offset":4836,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3167,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":4836,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3167,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":4836,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3167,"record_offset":5108,"parent_record_offset":5076,"ancestor_record_offset":4836,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3167,"record_offset":4980,"parent_record_offset":4952,"ancestor_record_offset":4836,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3167,"record_offset":4952,"parent_record_offset":4924,"ancestor_record_offset":4836,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4591,"record_offset":252,"rva":40418544,"va":5409127664}
// 编译实例 RVAs: 0x268bcf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_field
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2353485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409127664,"raw_hex":"e84de92300","size":5,"soft_fail":false,"target_va":5411481328,"va":5409127838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4591,"record_offset":432,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04591.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4591,"record_offset":456,"parent_record_offset":432,"ancestor_record_offset":252,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04591.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4591,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":252,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04591.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4591,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":252,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BB3
// 首条位置: {"module":4589,"inlinee":134067}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":32900,"parent_record_offset":32876,"ancestor_record_offset":32876,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BBB
// 首条位置: {"module":4589,"inlinee":134075}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":34120,"parent_record_offset":34092,"ancestor_record_offset":34092,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4589,"record_offset":34148,"parent_record_offset":34120,"ancestor_record_offset":34092,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":34204,"parent_record_offset":34120,"ancestor_record_offset":34092,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":34172,"parent_record_offset":34148,"ancestor_record_offset":34092,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BB5
// 首条位置: {"module":4589,"inlinee":134069}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":33252,"parent_record_offset":33228,"ancestor_record_offset":33228,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4589,"record_offset":33276,"parent_record_offset":33252,"ancestor_record_offset":33228,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":33332,"parent_record_offset":33252,"ancestor_record_offset":33228,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":33300,"parent_record_offset":33276,"ancestor_record_offset":33228,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20BB9
// 首条位置: {"module":4589,"inlinee":134073}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::deserialize_inheritable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4589,"record_offset":33712,"parent_record_offset":33684,"ancestor_record_offset":33684,"inlinee":134071,"source_line":158,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4589,"record_offset":33740,"parent_record_offset":33712,"ancestor_record_offset":33684,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4589,"record_offset":33796,"parent_record_offset":33712,"ancestor_record_offset":33684,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4589,"record_offset":33764,"parent_record_offset":33740,"ancestor_record_offset":33684,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04589.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::InheritableTextureOpts::inherit_from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A668
// 首条位置: {"module":3167,"inlinee":108136}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::Inheritable::inherit_shallow
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":6764,"parent_record_offset":6700,"ancestor_record_offset":6700,"inlinee":108137,"source_line":30,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$7::clone
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":6800,"parent_record_offset":6764,"ancestor_record_offset":6700,"inlinee":108139,"source_line":13,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3167,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":6700,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::InheritableTextureOpts::concrete_default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A65F
// 首条位置: {"module":3167,"inlinee":108127}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::inheritable::impl$6::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":3152,"parent_record_offset":3124,"ancestor_record_offset":3124,"inlinee":108129,"source_line":152,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   [pdb_inline_transitive] asset::inheritable::impl$2::concrete_default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":3167,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":3124,"inlinee":108131,"source_line":90,"archive":"_pdb/B/modules/03167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::config::InheritableTextureOpts::into_concrete
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4141,"record_offset":252,"rva":38101584,"va":5406810704}
// 编译实例 RVAs: 0x2456250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t4730976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406810704,"raw_hex":"e860304800","size":5,"soft_fail":false,"target_va":5411541904,"va":5406810923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::inheritable::Inheritable::into_concrete
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/inheritable.rs
//     evidence={"module":4141,"record_offset":400,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":125502,"source_line":53,"archive":"_pdb/B/modules/04141.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4141,"record_offset":444,"parent_record_offset":400,"ancestor_record_offset":252,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/04141.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4141,"record_offset":472,"parent_record_offset":400,"ancestor_record_offset":252,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04141.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::config::InheritableTextureOpts
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["alloc::string::String", "asset::texture::config::InheritableTextureOpts", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>", "constructor": "tuple$", "arguments": ["alloc::string::String", "asset::texture::config::InheritableTextureOpts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["alloc::string::String", "asset::texture::config::InheritableTextureOpts", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<str$,alloc::string::String,asset::texture::config::InheritableTextureOpts>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["str$", "alloc::string::String", "asset::texture::config::InheritableTextureOpts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<str$,alloc::string::String,asset::texture::config::InheritableTextureOpts> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<str$,alloc::string::String,asset::texture::config::InheritableTextureOpts>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<alloc::string::String,asset::texture::config::InheritableTextureOpts>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["alloc::string::String", "asset::texture::config::InheritableTextureOpts", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map::impl$0::fmt<serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map::impl$0::fmt", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<asset::texture::config::InheritableTextureOpts>", "constructor": "core::marker::PhantomData", "arguments": ["asset::texture::config::InheritableTextureOpts"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$6::next_value_seed<core::marker::PhantomData<asset::texture::config::InheritableTextureOpts> >", "constructor": "ron::de::impl$6::next_value_seed", "arguments": ["core::marker::PhantomData<asset::texture::config::InheritableTextureOpts>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<alloc::string::String,asset::texture::config::InheritableTextureOpts,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::config::TextureGroupConfig
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<asset::texture::config::TextureGroupConfig>", "constructor": "core::marker::PhantomData", "arguments": ["asset::texture::config::TextureGroupConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_str_seed<core::marker::PhantomData<asset::texture::config::TextureGroupConfig>,asset::texture::config::TextureGroupConfig>", "constructor": "ron::options::Options::from_str_seed", "arguments": ["core::marker::PhantomData<asset::texture::config::TextureGroupConfig>", "asset::texture::config::TextureGroupConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset::texture::config::InheritableTextureOpts::concrete_default
// 信息 ID: 6f3e6bbaf048899f740ba3e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree; module=3167, inline_record=3124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::InheritableTextureOpts::inherit_from
// 信息 ID: 99e357e28d6ded0ef4fbfb88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=6700; 不是独立调用边。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=16604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::InheritableTextureOpts::into_concrete
// 信息 ID: 4481dca95ca933811bfe351d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::texture::scan::scan_asset_tree_walker @ 0x20da09e -> 0x2456250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize
// 信息 ID: fa01c0883e4b71a6a0b9b633
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<asset::texture::config::TextureGroupConfig>,asset::texture::config::TextureGroupConfig>; module=4094, inline_record=556; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$6::next_value_seed<core::marker::PhantomData<asset::texture::config::InheritableTextureOpts> >; module=4589, inline_record=11096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 830048e62b7912d2689fbf57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=35820; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_identifier<asset::texture::config::_::impl$0::deserialize::__FieldVisitor>; module=4589, inline_record=59716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 130b2809820ec7dc12055d5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] asset::texture::config::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor> @ 0x268286c -> 0x268bcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<asset::texture::config::_::impl$0::deserialize::__FieldVisitor> @ 0x287d81a -> 0x268bcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<asset::texture::config::_::impl$0::deserialize::__FieldVisitor> @ 0x287d850 -> 0x268bcf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 0bca33a77a08246d38074458
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=32056; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=32220; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=32340; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=35392; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=35548; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=35676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: ba4859168f38edf702756203
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=31672; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=34980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$0::deserialize
// 信息 ID: c501f4ef87cca4c9ca05bd29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=32876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$1::deserialize
// 信息 ID: 906aabf553dc17a88380d164
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=34092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$2::deserialize
// 信息 ID: 9607486cb7f77b3d8bf19a31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=33228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::_::impl$0::deserialize::impl$2::visit_map::impl$3::deserialize
// 信息 ID: 36fa8cf996fc3079382544e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset::texture::config::_::impl$0::deserialize::__Visitor>; module=4589, inline_record=33684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::config::impl$2::default
// 信息 ID: cbebb463624d68761be6d46b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::scan::scan_asset_tree_walker; module=3167, inline_record=4836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
