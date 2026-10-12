// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-text/src/font_table.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f574bf591a470045720fef15e96d70042a98770478f4ffae939d945490e3e9a2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: font_table.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 349, 631, 632, 1236, 1239, 1249, 1308, 1876, 2887, 2890, 4229
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 5, 16, 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11211
// 首条位置: {"module":1239,"inlinee":70161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 5, 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B49B
// 首条位置: {"module":2887,"inlinee":111771}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2887,"record_offset":67624,"parent_record_offset":67584,"ancestor_record_offset":67584,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2887,"record_offset":67648,"parent_record_offset":67624,"ancestor_record_offset":67584,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":67672,"parent_record_offset":67648,"ancestor_record_offset":67584,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":67700,"parent_record_offset":67672,"ancestor_record_offset":67584,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 5, 16, 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B493
// 首条位置: {"module":2887,"inlinee":111763}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::id::impl$1::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":2887,"record_offset":90968,"parent_record_offset":90944,"ancestor_record_offset":90944,"inlinee":9960,"source_line":19,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":18316,"parent_record_offset":18292,"ancestor_record_offset":18292,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2887,"record_offset":110048,"parent_record_offset":110024,"ancestor_record_offset":109852,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":18352,"parent_record_offset":18316,"ancestor_record_offset":18292,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":18416,"parent_record_offset":18316,"ancestor_record_offset":18292,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":110072,"parent_record_offset":110048,"ancestor_record_offset":109852,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":110100,"parent_record_offset":110072,"ancestor_record_offset":109852,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2887,"record_offset":110024,"parent_record_offset":109984,"ancestor_record_offset":109852,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":90992,"parent_record_offset":90968,"ancestor_record_offset":90944,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":18388,"parent_record_offset":18316,"ancestor_record_offset":18292,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2887,"record_offset":109984,"parent_record_offset":109956,"ancestor_record_offset":109852,"inlinee":111771,"source_line":16,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 5, 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B49E
// 首条位置: {"module":2887,"inlinee":111774}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2887,"record_offset":91436,"parent_record_offset":90548,"ancestor_record_offset":90548,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2887,"record_offset":92076,"parent_record_offset":90548,"ancestor_record_offset":90548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":91332,"parent_record_offset":90548,"ancestor_record_offset":90548,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":90600,"parent_record_offset":90548,"ancestor_record_offset":90548,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":91372,"parent_record_offset":90548,"ancestor_record_offset":90548,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2887,"record_offset":93100,"parent_record_offset":93076,"ancestor_record_offset":90548,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2887,"record_offset":93124,"parent_record_offset":93100,"ancestor_record_offset":90548,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2887,"record_offset":93148,"parent_record_offset":93124,"ancestor_record_offset":90548,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2887,"record_offset":93076,"parent_record_offset":93020,"ancestor_record_offset":90548,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2887,"record_offset":93048,"parent_record_offset":93020,"ancestor_record_offset":90548,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2887,"record_offset":93020,"parent_record_offset":92996,"ancestor_record_offset":90548,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2887,"record_offset":92996,"parent_record_offset":92972,"ancestor_record_offset":90548,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":2887,"record_offset":90820,"parent_record_offset":90796,"ancestor_record_offset":90548,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2887,"record_offset":90848,"parent_record_offset":90820,"ancestor_record_offset":90548,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2887,"record_offset":110048,"parent_record_offset":110024,"ancestor_record_offset":109152,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2887,"record_offset":92364,"parent_record_offset":92340,"ancestor_record_offset":90548,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2887,"record_offset":93324,"parent_record_offset":93296,"ancestor_record_offset":90548,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2887,"record_offset":90888,"parent_record_offset":90640,"ancestor_record_offset":90548,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2887,"record_offset":93352,"parent_record_offset":93324,"ancestor_record_offset":90548,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2887,"record_offset":91296,"parent_record_offset":90640,"ancestor_record_offset":90548,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":2887,"record_offset":92584,"parent_record_offset":92560,"ancestor_record_offset":90548,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2887,"record_offset":92396,"parent_record_offset":92312,"ancestor_record_offset":90548,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2887,"record_offset":92100,"parent_record_offset":92076,"ancestor_record_offset":90548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2887,"record_offset":92852,"parent_record_offset":92828,"ancestor_record_offset":90548,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2887,"record_offset":92704,"parent_record_offset":92680,"ancestor_record_offset":90548,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":91148,"parent_record_offset":90640,"ancestor_record_offset":90548,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":90760,"parent_record_offset":90640,"ancestor_record_offset":90548,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":110072,"parent_record_offset":110048,"ancestor_record_offset":109152,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":110100,"parent_record_offset":110072,"ancestor_record_offset":109152,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2887,"record_offset":110024,"parent_record_offset":109984,"ancestor_record_offset":109152,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2887,"record_offset":92528,"parent_record_offset":92500,"ancestor_record_offset":90548,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2887,"record_offset":92560,"parent_record_offset":92528,"ancestor_record_offset":90548,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":2887,"record_offset":92500,"parent_record_offset":92460,"ancestor_record_offset":90548,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":2887,"record_offset":92340,"parent_record_offset":92312,"ancestor_record_offset":90548,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92828,"parent_record_offset":92256,"ancestor_record_offset":90548,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92680,"parent_record_offset":92460,"ancestor_record_offset":90548,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92312,"parent_record_offset":92288,"ancestor_record_offset":90548,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92460,"parent_record_offset":92432,"ancestor_record_offset":90548,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":93264,"parent_record_offset":93228,"ancestor_record_offset":90548,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92256,"parent_record_offset":92196,"ancestor_record_offset":90548,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92196,"parent_record_offset":92172,"ancestor_record_offset":90548,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":93228,"parent_record_offset":92196,"ancestor_record_offset":90548,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92228,"parent_record_offset":92196,"ancestor_record_offset":90548,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92288,"parent_record_offset":92256,"ancestor_record_offset":90548,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":93296,"parent_record_offset":93264,"ancestor_record_offset":90548,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92172,"parent_record_offset":92148,"ancestor_record_offset":90548,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":2887,"record_offset":92432,"parent_record_offset":92256,"ancestor_record_offset":90548,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":2887,"record_offset":109528,"parent_record_offset":109244,"ancestor_record_offset":109152,"inlinee":9971,"source_line":11,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::impl$1::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":2887,"record_offset":90968,"parent_record_offset":90944,"ancestor_record_offset":90548,"inlinee":9960,"source_line":19,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":110764,"parent_record_offset":110740,"ancestor_record_offset":109152,"inlinee":7159,"source_line":440,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":90992,"parent_record_offset":90968,"ancestor_record_offset":90548,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":90640,"parent_record_offset":90600,"ancestor_record_offset":90548,"inlinee":9956,"source_line":852,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":91500,"parent_record_offset":91464,"ancestor_record_offset":90548,"inlinee":9908,"source_line":873,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_content
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":91688,"parent_record_offset":91500,"ancestor_record_offset":90548,"inlinee":9910,"source_line":1050,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_tag_or_content
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":90796,"parent_record_offset":90640,"ancestor_record_offset":90548,"inlinee":9920,"source_line":1109,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":109956,"parent_record_offset":109880,"ancestor_record_offset":109152,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":90920,"parent_record_offset":90640,"ancestor_record_offset":90548,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":2887,"record_offset":91836,"parent_record_offset":91812,"ancestor_record_offset":90548,"inlinee":83339,"source_line":1515,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":2887,"record_offset":110740,"parent_record_offset":110716,"ancestor_record_offset":109152,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2887,"record_offset":109984,"parent_record_offset":109956,"ancestor_record_offset":109152,"inlinee":111771,"source_line":16,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":2887,"record_offset":90944,"parent_record_offset":90920,"ancestor_record_offset":90548,"inlinee":111763,"source_line":5,"archive":"_pdb/C/modules/02887.jsonl.gz"}
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
// 首条位置: {"module":632,"record_offset":264,"rva":9525504,"va":5378234624}
// 编译实例 RVAs: 0x915900
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5633
// 首条位置: {"module":349,"inlinee":22067}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":349,"record_offset":4296,"parent_record_offset":4268,"ancestor_record_offset":4268,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":349,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":4268,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":349,"record_offset":4724,"parent_record_offset":4520,"ancestor_record_offset":4268,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":349,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":4268,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":349,"record_offset":4848,"parent_record_offset":4796,"ancestor_record_offset":4268,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":349,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":4268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":349,"record_offset":4552,"parent_record_offset":4520,"ancestor_record_offset":4268,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":349,"record_offset":4640,"parent_record_offset":4520,"ancestor_record_offset":4268,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":349,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":4268,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":349,"record_offset":4352,"parent_record_offset":4324,"ancestor_record_offset":4268,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":349,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":4268,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":349,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":4268,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":349,"record_offset":4796,"parent_record_offset":4408,"ancestor_record_offset":4268,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":349,"record_offset":4324,"parent_record_offset":4296,"ancestor_record_offset":4268,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":349,"record_offset":4492,"parent_record_offset":4464,"ancestor_record_offset":4268,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":349,"record_offset":4520,"parent_record_offset":4492,"ancestor_record_offset":4268,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":349,"record_offset":4408,"parent_record_offset":4380,"ancestor_record_offset":4268,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":349,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":4268,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00349.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::FontTable::get
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A4E
// 首条位置: {"module":631,"inlinee":39502}
// 编译实例 RVAs: 0xcb2cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d580
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: font_table.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8542107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382024432,"raw_hex":"e865a87dff","size":5,"soft_fail":false,"target_va":5373482368,"va":5382024470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":631,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":3288,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":631,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":3288,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":631,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":3288,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":631,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":3288,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":631,"record_offset":4072,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":631,"record_offset":3804,"parent_record_offset":3776,"ancestor_record_offset":3288,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":631,"record_offset":3776,"parent_record_offset":3748,"ancestor_record_offset":3288,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":631,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":3288,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":631,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":3288,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":631,"record_offset":3716,"parent_record_offset":3688,"ancestor_record_offset":3288,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":631,"record_offset":3748,"parent_record_offset":3716,"ancestor_record_offset":3288,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":631,"record_offset":3688,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":631,"record_offset":3548,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":631,"record_offset":4100,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":631,"record_offset":3632,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":631,"record_offset":3492,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":631,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":3288,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":631,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":3288,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":631,"record_offset":3956,"parent_record_offset":3848,"ancestor_record_offset":3288,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":631,"record_offset":3364,"parent_record_offset":3336,"ancestor_record_offset":3288,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":631,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":3288,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":631,"record_offset":4192,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":631,"record_offset":3872,"parent_record_offset":3848,"ancestor_record_offset":3288,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":631,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":3288,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":631,"record_offset":3392,"parent_record_offset":3336,"ancestor_record_offset":3288,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":631,"record_offset":3440,"parent_record_offset":3416,"ancestor_record_offset":3288,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":631,"record_offset":3520,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":631,"record_offset":3848,"parent_record_offset":3440,"ancestor_record_offset":3288,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00631.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::font_table::impl$7::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/font_table.rs
//     evidence={"module":631,"record_offset":4028,"parent_record_offset":4004,"ancestor_record_offset":3288,"inlinee":39500,"source_line":27,"archive":"_pdb/C/modules/00631.jsonl.gz"}
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
// 首条位置: {"module":2890,"record_offset":264,"rva":36984064,"va":5405693184}
// 编译实例 RVAs: 0x2345500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10862416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405693184,"raw_hex":"e850bfa500","size":5,"soft_fail":false,"target_va":5416555792,"va":5405693371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2890,"record_offset":440,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02890.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2890,"record_offset":468,"parent_record_offset":440,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02890.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2890,"record_offset":496,"parent_record_offset":468,"ancestor_record_offset":264,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02890.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2890,"record_offset":528,"parent_record_offset":496,"ancestor_record_offset":264,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02890.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B49D
// 首条位置: {"module":2887,"inlinee":111773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":81532,"parent_record_offset":81480,"ancestor_record_offset":81480,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":81504,"parent_record_offset":81480,"ancestor_record_offset":81480,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A4C
// 首条位置: {"module":631,"inlinee":39500}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::impl$9::hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111EA
// 首条位置: {"module":1236,"inlinee":70122}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1236,"record_offset":608,"parent_record_offset":588,"ancestor_record_offset":588,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01236.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1236,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":588,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01236.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1236,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":588,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01236.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1236,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":588,"inlinee":6724,"source_line":232,"archive":"_pdb/C/modules/01236.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1236,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":588,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01236.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1236,"record_offset":708,"parent_record_offset":656,"ancestor_record_offset":588,"inlinee":6722,"source_line":130,"archive":"_pdb/C/modules/01236.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::font_table::FontTableEntry::scaled
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A50
// 首条位置: {"module":631,"inlinee":39504}
// 编译实例 RVAs: 0xcb2cd0
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "ref$<tg_text::font_table::FontFamily>", "constructor": "ref$", "arguments": ["tg_text::font_table::FontFamily"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<tg_text::font_table::FontFamily>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "constructor": "tuple$", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "serde_core::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde_core::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::font_table::FontTable
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$62::default<tg_text::font_table::FontTable>", "constructor": "alloc::sync::impl$62::default", "arguments": ["tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<tg_text::font_table::FontTable,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["tg_text::font_table::FontTable", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<tg_text::font_table::FontTable>", "constructor": "alloc::sync::ArcInner", "arguments": ["tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<tg_text::font_table::FontTable> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<tg_text::font_table::FontTable>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<tg_text::font_table::FontTable>", "constructor": "core::marker::PhantomData", "arguments": ["tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_str_seed<core::marker::PhantomData<tg_text::font_table::FontTable>,tg_text::font_table::FontTable>", "constructor": "ron::options::Options::from_str_seed", "arguments": ["core::marker::PhantomData<tg_text::font_table::FontTable>", "tg_text::font_table::FontTable"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::font_table::FontTableEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "constructor": "tuple$", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>", "constructor": "serde_core::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["tg_text::font_table::FontFamily", "tg_text::font_table::FontTableEntry", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde_core::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>

// ------------------------------------------------------------------
// 函数: tg_text::font_table::FontTable::get
// 信息 ID: c9c1f62fc56430c384044c9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x607d54 -> 0xcb2cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x11a38a4 -> 0xcb2cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::new<f32> @ 0x11a3984 -> 0xcb2cf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x2b14094 -> 0xcb2cf0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=631, inline_record=3288; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::FontTableEntry::scaled
// 信息 ID: 8c3de7633ee8d798b7cacacd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x607d6b -> 0xcb2cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x11a38bb -> 0xcb2cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::new<f32> @ 0x11a39a4 -> 0xcb2cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextFragment::set_metrics<f32> @ 0x2b140ab -> 0xcb2cd0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=631, inline_record=4248; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize
// 信息 ID: c30b08c5043cbf6e64f2f7ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<tg_text::font_table::FontTable>,tg_text::font_table::FontTable>; module=1308, inline_record=544; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >; module=2887, inline_record=53776; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde_core::de::impls::impl$204::deserialize::MapVisitor<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState> >; module=2887, inline_record=54196; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<toolbar::mei::config::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=117264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 6fd1d753d1cce17f2f7927e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_str<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=2887, inline_record=67584; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_str<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=2887, inline_record=67932; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_str<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=2887, inline_record=71816; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_str<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=2887, inline_record=72020; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=109984; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::font_table::_::impl$0::deserialize::__FieldVisitor>; module=4229, inline_record=564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 14dd507aa0ba5f95fa609741
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] tg_text::font_table::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::font_table::_::impl$0::deserialize::__Field> > @ 0x2330f56 -> 0x2345500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: e57f49f159b862d4440329e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::font_table::_::impl$0::deserialize::__Field> >; module=2887, inline_record=18292; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=90944; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=91088; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=91236; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=109580; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=109696; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=109852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 7c86a507ee39a4b4b3fff149
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=81480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 7cbb0c3ca1ef41b23939aaeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=90548; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=109152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::default_scale
// 信息 ID: 59dd72d21c62ef36c762a04d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<tg_text::font_table::_::impl$0::deserialize::__Visitor> @ 0x233edca -> 0x915900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::show_color_menu::show_color_options @ 0x289d0a1 -> 0x915900 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::show_color_menu::show_color_options @ 0x289d4c0 -> 0x915900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$2::default
// 信息 ID: ec70c402320daa433fe4c4a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$62::default<tg_text::font_table::FontTable>; module=349, inline_record=4268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$7::eq
// 信息 ID: ef06048cf03b88928cfe6a6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=631, inline_record=3172; 不是独立调用边。
// [PDB 内联宿主] tg_text::TextFragment::set_family; module=631, inline_record=4028; 不是独立调用边。
// [PDB 内联宿主] tg_text::font_table::FontTable::get; module=632, inline_record=1336; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<tg_text::font_table::FontFamily,tg_text::font_table::FontTableEntry,std::hash::random::RandomState,alloc::alloc::Global>; module=1249, inline_record=64408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::font_table::impl$9::hash
// 信息 ID: 4912846a7d1cb5efc3b47ac9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >; module=1236, inline_record=588; 不是独立调用边。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_text::font_table::FontFamily> >; module=1876, inline_record=7832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
